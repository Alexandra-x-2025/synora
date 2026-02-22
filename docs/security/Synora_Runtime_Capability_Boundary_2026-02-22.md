# Synora -- Runtime Capability Boundary (2026-02-22)

Purpose:
- Provide a single source of truth for what Synora can and cannot do in current runtime.

---

## 1. Available Now

- Software discovery and update check via CLI.
- Plan/audit path for update apply.
- Cleanup quarantine simulation lifecycle with audit persistence.
- Security enforcement for cleanup path:
- path traversal blocking
- symlink escape blocking
- allowlist root validation
- high/critical confirmation gate
- Audit visibility:
- `config history-list`
- `config audit-summary`

---

## 2. Still Gated

- Real file mutation.
- Real registry mutation.
- Real rollback mutation.
- Privilege elevation path.

---

## 3. Gate Status

- Simulation path: Go.
- Real mutation path: No-Go.

No-Go conditions:
- Security sign-off not finalized.
- Real mutation gate approval not finalized.
- Runtime gate remains default disabled.

---

## 4. Gate Control References

- `docs/security/Synora_Quarantine_Execution_Design.md`
- `docs/security/Synora_Real_Mutation_Gate_Strategy_Draft.md`
- `docs/security/Synora_Real_Mutation_Gate_Approval_Record_2026-02-22_Draft.md`
- `docs/security/Synora_Real_Mutation_Gate_Final_Signoff_Pack.md`

---

## 5. External Communication Rule

When describing Synora publicly:
- Do not claim real system mutation is enabled.
- Explicitly state current behavior is simulation+audit first.
- Point to sign-off documents for go-live progress.

