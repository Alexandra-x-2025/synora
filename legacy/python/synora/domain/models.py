from __future__ import annotations

from dataclasses import asdict, dataclass


@dataclass(slots=True)
class SoftwareItem:
    name: str
    package_id: str
    version: str
    source: str

    def to_dict(self) -> dict[str, str]:
        return asdict(self)


@dataclass(slots=True)
class UpdateItem:
    name: str
    package_id: str
    installed_version: str
    available_version: str
    source: str

    def to_dict(self) -> dict[str, str]:
        return asdict(self)
