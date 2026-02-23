#!/usr/bin/env python3
import json
import os
import subprocess
import sys
import urllib.parse
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
UI_DIR = ROOT / "ui"
DEFAULT_PORT = 8787


def synora_bin() -> str:
    exe = ROOT / "target" / "debug" / ("synora.exe" if os.name == "nt" else "synora")
    if exe.exists():
        return str(exe)
    return "cargo"


def run_synora(args):
    bin_path = synora_bin()
    if bin_path == "cargo":
        cmd = ["cargo", "run", "--quiet", "--"] + args
    else:
        cmd = [bin_path] + args
    proc = subprocess.run(
        cmd,
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    return proc.returncode, proc.stdout.strip(), proc.stderr.strip(), cmd


class SynoraUiHandler(SimpleHTTPRequestHandler):
    def translate_path(self, path):
        parsed = urllib.parse.urlparse(path)
        request_path = parsed.path
        if request_path == "/":
            request_path = "/index.html"
        rel = request_path.lstrip("/")
        return str((UI_DIR / rel).resolve())

    def _json(self, status, payload):
        body = json.dumps(payload).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path == "/api/search":
            q = urllib.parse.parse_qs(parsed.query).get("q", [""])[0].strip()
            if not q:
                self._json(400, {"error": "--q is required"})
                return
            code, stdout, stderr, cmd = run_synora(["ui", "search", "--q", q, "--json"])
            if code != 0:
                self._json(
                    500,
                    {
                        "error": "ui search failed",
                        "command": cmd,
                        "stderr": stderr,
                        "exit_code": code,
                    },
                )
                return
            try:
                payload = json.loads(stdout)
            except json.JSONDecodeError:
                self._json(
                    500,
                    {"error": "invalid json from synora", "stdout": stdout, "stderr": stderr},
                )
                return
            self._json(200, payload)
            return
        super().do_GET()

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", "0"))
        raw = self.rfile.read(length).decode("utf-8") if length > 0 else ""
        try:
            body = json.loads(raw) if raw else {}
        except json.JSONDecodeError:
            self._json(400, {"error": "invalid json body"})
            return

        if parsed.path == "/api/action-run":
            action_id = str(body.get("id", "")).strip()
            confirm = bool(body.get("confirm", False))
            if not action_id:
                self._json(400, {"error": "--id is required"})
                return
            args = ["ui", "action-run", "--id", action_id, "--json"]
            if confirm:
                args.insert(4, "--confirm")
            code, stdout, stderr, cmd = run_synora(args)
            if code != 0:
                self._json(
                    200,
                    {
                        "ok": False,
                        "command": cmd,
                        "stderr": stderr,
                        "exit_code": code,
                    },
                )
                return
            try:
                payload = json.loads(stdout)
            except json.JSONDecodeError:
                self._json(
                    500,
                    {"ok": False, "error": "invalid json from synora", "stdout": stdout},
                )
                return
            self._json(200, {"ok": True, "result": payload, "exit_code": code})
            return

        if parsed.path == "/api/op":
            op = str(body.get("op", "")).strip()
            payload = body.get("payload") if isinstance(body.get("payload"), dict) else {}
            op_args = {
                "update_check": ["update", "check", "--json", "--limit", "20"],
                "discover_scan": ["software", "discover", "scan", "--json"],
                "repo_sync": ["repo", "sync", "--json"],
                "ai_analyze": ["ai", "analyze", "--json"],
                "ai_recommend": [
                    "ai",
                    "recommend",
                    "--goal",
                    str(payload.get("goal", "Rust development workstation")),
                    "--json",
                ],
                "ai_repair_plan": [
                    "ai",
                    "repair-plan",
                    "--software",
                    str(payload.get("software", "PowerToys")),
                    "--issue",
                    str(payload.get("issue", "crash on launch after update")),
                    "--json",
                ],
            }
            args = op_args.get(op)
            if not args:
                self._json(400, {"error": "unknown op"})
                return
            code, stdout, stderr, cmd = run_synora(args)
            if code != 0:
                self._json(
                    200,
                    {
                        "ok": False,
                        "command": cmd,
                        "stderr": stderr,
                        "exit_code": code,
                    },
                )
                return
            parsed_result = None
            try:
                parsed_result = json.loads(stdout) if stdout else None
            except json.JSONDecodeError:
                parsed_result = {"raw": stdout}
            self._json(200, {"ok": True, "result": parsed_result, "exit_code": code, "command": cmd})
            return

        self.send_error(404)


def main():
    port = int(os.environ.get("SYNORA_UI_PORT", DEFAULT_PORT))
    os.chdir(UI_DIR)
    server = ThreadingHTTPServer(("127.0.0.1", port), SynoraUiHandler)
    print(f"[synora-ui] serving ui at http://127.0.0.1:{port}")
    print("[synora-ui] api: GET /api/search?q=<query>, POST /api/action-run, POST /api/op")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[synora-ui] stopped")
    finally:
        server.server_close()


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:
        print(f"[synora-ui] fatal: {exc}", file=sys.stderr)
        raise
