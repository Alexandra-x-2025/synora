# Synora -- Security Threat Model

# Synora -- 安全威胁建模

Generated on: 2026-02-21 22:37:46

------------------------------------------------------------------------

## Threat Categories / 威胁类别

1.  Malicious Installer Injection
2.  Registry Tampering
3.  Orphan Cleanup Overreach
4.  Privilege Escalation Attempt

------------------------------------------------------------------------

## Mitigation Strategies / 缓解策略

-   Strict whitelist validation
-   Registry snapshot rollback
-   Quarantine staging
-   Explicit confirmation for high-risk actions

------------------------------------------------------------------------

## Risk Levels / 风险等级

LOW -- Read-only operations\
MEDIUM -- Update operations\
HIGH -- Registry modifications\
CRITICAL -- System-level installer execution
