from __future__ import annotations

import io
import unittest
from contextlib import redirect_stdout
from unittest.mock import patch

from synora import cli
from synora.security.guard import CommandResult


class CliTests(unittest.TestCase):
    def test_parser_requires_command(self) -> None:
        code = cli.main([])
        self.assertEqual(code, cli.EXIT_USAGE)

    def test_update_apply_plan_mode(self) -> None:
        code = cli.main(["update", "apply", "--id", "Git.Git"])
        self.assertEqual(code, cli.EXIT_OK)

    def test_update_apply_confirm_mode(self) -> None:
        code = cli.main(["update", "apply", "--id", "Git.Git", "--confirm", "--json"])
        self.assertEqual(code, cli.EXIT_OK)

    def test_update_apply_yes_alias(self) -> None:
        code = cli.main(["update", "apply", "--id", "Git.Git", "--yes"])
        self.assertEqual(code, cli.EXIT_OK)

    def test_update_apply_json_contract_fields(self) -> None:
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            code = cli.main(["update", "apply", "--id", "Git.Git", "--confirm", "--json"])
        self.assertEqual(code, cli.EXIT_OK)
        payload = stdout.getvalue()
        self.assertIn('"package_id": "Git.Git"', payload)
        self.assertIn('"requested_mode": "confirm"', payload)
        self.assertIn('"mode": "confirmed-plan"', payload)

    @patch("synora.integration.winget_client.WingetClient._supported", return_value=True)
    @patch("synora.security.guard.SecurityGuard.run")
    def test_update_check_integration_failure_exit_code(self, mock_run, _mock_supported) -> None:
        mock_run.return_value = CommandResult(code=1, stdout="", stderr="winget failed")
        code = cli.main(["update", "check"])
        self.assertEqual(code, cli.EXIT_INTEGRATION)

    @patch("synora.integration.winget_client.WingetClient._supported", return_value=True)
    @patch("synora.security.guard.SecurityGuard.run")
    def test_software_list_integration_failure_exit_code(self, mock_run, _mock_supported) -> None:
        mock_run.return_value = CommandResult(code=2, stdout="", stderr="winget failed")
        code = cli.main(["software", "list"])
        self.assertEqual(code, cli.EXIT_INTEGRATION)

    def test_update_apply_conflicting_mode_flags_returns_usage(self) -> None:
        code = cli.main(["update", "apply", "--id", "Git.Git", "--dry-run", "--confirm"])
        self.assertEqual(code, cli.EXIT_USAGE)

    def test_update_apply_missing_id_returns_usage(self) -> None:
        code = cli.main(["update", "apply"])
        self.assertEqual(code, cli.EXIT_USAGE)


if __name__ == "__main__":
    unittest.main()
