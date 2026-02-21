from __future__ import annotations

import unittest

from synora.security.guard import SecurityError, SecurityGuard


class SecurityGuardTests(unittest.TestCase):
    def setUp(self) -> None:
        self.guard = SecurityGuard()

    def test_allows_winget_list(self) -> None:
        self.guard.validate(["winget", "list", "--output", "json"])

    def test_rejects_non_allowlisted_program(self) -> None:
        with self.assertRaises(SecurityError):
            self.guard.validate(["powershell", "-Command", "Get-Process"])

    def test_rejects_non_allowlisted_operation(self) -> None:
        with self.assertRaises(SecurityError):
            self.guard.validate(["winget", "install", "Git.Git"])


if __name__ == "__main__":
    unittest.main()
