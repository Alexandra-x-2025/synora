from __future__ import annotations

import argparse
import json
import logging
from dataclasses import asdict

from synora.integration.winget_client import WingetClient
from synora.logging_utils import setup_logging
from synora.repository.config_repository import ConfigRepository
from synora.security.guard import SecurityError, SecurityGuard
from synora.service.software_service import SoftwareService
from synora.service.update_service import UpdateService
from synora.worker.task_engine import TaskEngine

EXIT_OK = 0
EXIT_USAGE = 2
EXIT_SECURITY = 3
EXIT_INTEGRATION = 4
EXIT_INTERNAL = 10


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="synora", description="Synora CLI v0.1")
    subparsers = parser.add_subparsers(dest="command")

    software_parser = subparsers.add_parser("software", help="Software operations")
    software_subparsers = software_parser.add_subparsers(dest="software_command")
    software_list = software_subparsers.add_parser("list", help="List installed software")
    software_list.add_argument("--json", action="store_true", dest="as_json", help="Output JSON")

    update_parser = subparsers.add_parser("update", help="Update operations")
    update_subparsers = update_parser.add_subparsers(dest="update_command")
    update_check = update_subparsers.add_parser("check", help="Check for updates")
    update_check.add_argument("--json", action="store_true", dest="as_json", help="Output JSON")

    update_apply = update_subparsers.add_parser("apply", help="Apply update plan for a package")
    update_apply.add_argument("--id", required=True, dest="package_id", help="Package identifier")
    mode_group = update_apply.add_mutually_exclusive_group()
    mode_group.add_argument(
        "--dry-run",
        action="store_true",
        dest="dry_run",
        help="Generate plan only (default behavior)",
    )
    mode_group.add_argument(
        "--confirm",
        "--yes",
        action="store_true",
        dest="confirmed",
        help="Confirm high-risk action",
    )
    update_apply.add_argument("--json", action="store_true", dest="as_json", help="Output JSON")

    config_parser = subparsers.add_parser("config", help="Configuration operations")
    config_subparsers = config_parser.add_subparsers(dest="config_command")
    config_subparsers.add_parser("init", help="Initialize default configuration")

    return parser


def _format_table(rows: list[dict[str, str]], columns: list[str]) -> str:
    if not rows:
        return "No entries found."

    widths = {col: len(col) for col in columns}
    for row in rows:
        for col in columns:
            widths[col] = max(widths[col], len(str(row.get(col, ""))))

    header = "  ".join(col.ljust(widths[col]) for col in columns)
    sep = "  ".join("-" * widths[col] for col in columns)
    body = ["  ".join(str(row.get(col, "")).ljust(widths[col]) for col in columns) for row in rows]
    return "\n".join([header, sep, *body])


def _handle_software_list(software_service: SoftwareService, as_json: bool) -> int:
    items = [asdict(item) for item in software_service.list_software()]
    if as_json:
        print(json.dumps(items, ensure_ascii=True, indent=2))
        return EXIT_OK

    print(_format_table(items, ["name", "package_id", "version", "source"]))
    return EXIT_OK


def _handle_update_check(update_service: UpdateService, as_json: bool) -> int:
    items = [asdict(item) for item in update_service.check_updates()]
    if as_json:
        print(json.dumps(items, ensure_ascii=True, indent=2))
        return EXIT_OK

    print(
        _format_table(
            items,
            ["name", "package_id", "installed_version", "available_version", "source"],
        )
    )
    return EXIT_OK


def _handle_update_apply(
    update_service: UpdateService,
    package_id: str,
    confirmed: bool,
    dry_run: bool,
    as_json: bool,
) -> int:
    risk = update_service.plan_apply(package_id=package_id, confirmed=confirmed)
    mode = "confirmed-plan" if confirmed else "plan-only"
    requested_mode = "confirm" if confirmed else "dry-run"
    payload = {
        "package_id": package_id,
        "risk": risk.value,
        "confirmed": confirmed,
        "dry_run": dry_run or not confirmed,
        "requested_mode": requested_mode,
        "mode": mode,
        "message": "v0.1 does not execute real updates yet",
    }
    if as_json:
        print(json.dumps(payload, ensure_ascii=True, indent=2))
    else:
        print(f"Package: {package_id}")
        print(f"Risk: {risk.value}")
        print(f"Requested Mode: {requested_mode}")
        print(f"Mode: {mode}")
        print("Note: v0.1 does not execute real updates yet")
    return EXIT_OK


def _handle_config_init(config_repo: ConfigRepository) -> int:
    path = config_repo.init_default()
    print(f"Config initialized: {path}")
    return EXIT_OK


def main(argv: list[str] | None = None) -> int:
    log_path = setup_logging()
    logger = logging.getLogger("synora")
    parser = build_parser()
    try:
        args = parser.parse_args(argv)
    except SystemExit as exc:
        return EXIT_OK if exc.code == 0 else EXIT_USAGE

    if not args.command:
        parser.print_help()
        return EXIT_USAGE

    guard = SecurityGuard()
    winget = WingetClient(guard=guard)
    software_service = SoftwareService(winget=winget)
    update_service = UpdateService(winget=winget)
    config_repo = ConfigRepository()
    task_engine = TaskEngine()

    try:
        if args.command == "software" and args.software_command == "list":
            return task_engine.run_with_retry(lambda: _handle_software_list(software_service, args.as_json))

        if args.command == "update" and args.update_command == "check":
            return task_engine.run_with_retry(lambda: _handle_update_check(update_service, args.as_json))

        if args.command == "update" and args.update_command == "apply":
            confirmed = bool(getattr(args, "confirmed", False))
            dry_run = bool(getattr(args, "dry_run", False))
            return task_engine.run_with_retry(
                lambda: _handle_update_apply(update_service, args.package_id, confirmed, dry_run, args.as_json)
            )

        if args.command == "config" and args.config_command == "init":
            return task_engine.run_with_retry(lambda: _handle_config_init(config_repo))

        parser.print_help()
        return EXIT_USAGE
    except ValueError as exc:
        logger.error("Validation error: %s", exc)
        print(f"Validation error: {exc}")
        return EXIT_USAGE
    except SecurityError as exc:
        logger.error("Security blocked: %s", exc)
        print(f"Security blocked: {exc}")
        return EXIT_SECURITY
    except OSError as exc:
        logger.error("Integration failure: %s", exc)
        print(f"Integration failure: {exc}")
        return EXIT_INTEGRATION
    except Exception as exc:  # pragma: no cover
        logger.exception("Unexpected internal error")
        print(f"Unexpected error. See log: {log_path}. Details: {exc}")
        return EXIT_INTERNAL


if __name__ == "__main__":
    raise SystemExit(main())
