# Synora Push Workflow Standard

Date baseline: 2026-02-22
Scope: Local commit and push process for this repository

## 1. Pre-Push Checks

Run before every push:

1. `git status --short`
2. `git status -sb`
3. `git log --oneline --decorate -n 10`

If code changed, also run:

1. `cargo check`
2. `cargo test`

Docs-only fast path:

Use when staged files are documentation-only (for example `*.md`, `docs/**`, governance docs).

1. `git status --short`
2. `git diff --name-only --cached` (verify docs-only scope)
3. `git log --oneline --decorate -n 10`

No `cargo check` / `cargo test` required in docs-only fast path.

## 2. Staging Rules

- Stage only files related to the target change.
- Do not stage editor/runtime artifacts unless explicitly required.
- Default exclude:
- `.vscode/`
- local runtime directories (for example `.synora*/`)

## 3. Commit Message Convention

Use:

`<type>(<scope>): <summary>`

Types:

- `feat`: new behavior/capability
- `fix`: bug fix
- `test`: tests only
- `docs`: docs only
- `chore`: maintenance/process/meta updates

Examples:

- `feat(phase2-week1): add sqlite repository baseline and schema bootstrap`
- `docs(phase1): add rust quick start and mvp readiness checklist`
- `chore(phase2): mark phase1 complete and start phase2 planning`

## 4. Push Command Template

1. Stage

```powershell
git add <files...>
```

2. Commit

```powershell
git commit -m "<type>(<scope>): <summary>"
```

3. Push

```powershell
git push origin main
```

If not on `main`:

```powershell
git push origin <your-branch>
```

## 5. Assistant Output Contract

For every requested push, assistant should provide:

1. Current `git status` summary
2. Recommended commit message
3. Exact `git add` command with explicit file list
4. Exact `git commit` command
5. Exact `git push` command
6. Branch note if branch is not `main`

Assistant mode selection rule:

- If staged scope includes code/config/test files, use `Code change push` template.
- If staged scope is docs-only, use `Docs-only push` template.
- When in doubt, default to `Code change push`.

### Code change push template

Use this output order:

1. `git status --short` summary
2. Verification checklist:
- `cargo check`
- `cargo test`
3. Recommended commit message
4. Exact commands:

```powershell
git add <explicit-file-list>
git commit -m "<type>(<scope>): <summary>"
git push origin <branch>
```

### Docs-only push template

Use this output order:

1. `git status --short` summary
2. Docs-only scope proof:
- `git diff --name-only --cached`
3. Recommended commit message (`docs` or `chore`)
4. Exact commands:

```powershell
git add <explicit-file-list>
git commit -m "<type>(<scope>): <summary>"
git push origin <branch>
```

Note:
- Do not require `cargo check` / `cargo test` in docs-only template.

## 6. Safety Rules

- Never use destructive git commands (`reset --hard`, checkout discard) unless explicitly requested.
- If unrelated modified files exist, stage only scoped files.
- If push includes phase transition/state changes, include matching updates in:
- `PROJECT_STATE.md`
- `DEVELOPMENT_LOG.md`
- `docs/roadmap.md` (when applicable)
