# Discovery Scope Decision (One Pager)

Date: 2026-02-22
Status: Approved
Scope: Synora Software Discovery in MVP

## 1. Decision Question
Should MVP Discovery use:
1. Registry-only
2. Registry + Program Files + Start Menu (multi-source)

## 2. Options

### Option A: Registry-only (HKLM/HKCU Uninstall)

Coverage:
- Installed software with proper uninstall records

Pros:
- Fastest to implement
- Lowest complexity
- Stable parsing and fewer false positives
- Easiest to audit and debug

Cons:
- Misses portable apps
- Misses some non-standard installers

Engineering Impact:
- Low

Timeline Impact:
- Minimal

Risk:
- Lower detection coverage, but predictable quality

---

### Option B: Multi-source (Registry + Program Files + Start Menu)

Coverage:
- Broader discovery (including some non-standard installs)

Pros:
- Better user-perceived completeness
- Stronger foundation for AI recommendation and link enrichment

Cons:
- Requires dedup/merge/confidence model in MVP
- More false positives and noisy records
- Higher testing and debugging overhead

Engineering Impact:
- Medium to high

Timeline Impact:
- Significant (MVP schedule pressure)

Risk:
- Higher complexity may delay core contract freeze

## 3. Recommendation
Recommended for MVP: **Option A (Registry-only)**

Reason:
- Keep MVP focused on safety and controllable execution.
- Freeze architecture/API/data model quickly with lower uncertainty.
- Add Option B as Phase 2 enhancement after core workflow is stable.

## 4. If Option A is chosen
MVP constraints:
1. Discovery source fixed to Registry uninstall keys.
2. Missing software classes are explicitly documented as known gap.
3. Data model keeps extension points (`source`, `confidence`) for Phase 2.

Phase 2 entry:
1. Add Program Files scanner.
2. Add Start Menu scanner.
3. Introduce merge and confidence rules.

## 5. Decision Record
- Selected Option: [x] A Registry-only  [ ] B Multi-source
- Decision Owner: User + Codex
- Effective Date: 2026-02-22
- Rationale Notes: Freeze MVP on lower-risk, lower-complexity path; defer multi-source
  merge/confidence model to Phase 2.

## 6. Downstream Docs to Update After Decision
- `docs/PRODUCT_SPEC.md`
- `docs/FEATURES.md`
- `docs/ROADMAP.md`
- `docs/ARCHITECTURE.md`
- `docs/DATA_MODEL.md`
- `docs/API_SPEC.md`
- `logs/DEVELOPMENT_LOG.md`
