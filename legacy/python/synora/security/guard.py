from __future__ import annotations

import shlex
import subprocess
from dataclasses import dataclass


@dataclass(slots=True)
class CommandResult:
    code: int
    stdout: str
    stderr: str


class SecurityError(RuntimeError):
    pass


class SecurityGuard:
    """Single boundary for system-level operations."""

    _allowed_commands = {
        "winget": {
            "list",
            "upgrade",
        }
    }

    def validate(self, command: list[str]) -> None:
        if not command:
            raise SecurityError("Empty command is not allowed")

        program = command[0]
        if program not in self._allowed_commands:
            raise SecurityError(f"Program '{program}' is not allowlisted")

        operation = next((part for part in command[1:] if not part.startswith("-")), "")
        if operation not in self._allowed_commands[program]:
            raise SecurityError(f"Operation '{operation}' is not allowlisted for {program}")

    def run(self, command: list[str]) -> CommandResult:
        self.validate(command)
        completed = subprocess.run(command, capture_output=True, text=True, check=False)
        return CommandResult(code=completed.returncode, stdout=completed.stdout, stderr=completed.stderr)

    def run_safe_string(self, command: str) -> CommandResult:
        parsed = shlex.split(command)
        return self.run(parsed)
