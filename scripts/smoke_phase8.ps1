$ErrorActionPreference = 'Stop'

function Invoke-CargoStrict {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Args,
        [int[]]$AllowedExitCodes = @(0)
    )

    & cargo @Args
    $code = $LASTEXITCODE
    if ($AllowedExitCodes -notcontains $code) {
        throw "cargo $($Args -join ' ') failed with exit code $code"
    }
}

Write-Host '[phase8-smoke] build checks'
Invoke-CargoStrict @('check')
Invoke-CargoStrict @('test')

Write-Host '[phase8-smoke] core pipeline'
Invoke-CargoStrict @('run','--','config','init','--json')
Invoke-CargoStrict @('run','--','software','discover','scan','--json')
Invoke-CargoStrict @('run','--','source','suggest','--json','--limit','5')
Invoke-CargoStrict @('run','--','source','review-bulk','--approve','--status','pending','--domain','github.com','--limit','1','--json')
Invoke-CargoStrict @('run','--','source','apply-approved','--json','--limit','1')

Write-Host '[phase8-smoke] update + cleanup guarded execution'
Invoke-CargoStrict @('run','--','update','apply','--candidate-id','180','--dry-run','--json')
Invoke-CargoStrict @('run','--','update','apply','--candidate-id','180','--confirm','--execution-ticket','phase8-ticket-update-001','--json')
Invoke-CargoStrict @('run','--','cleanup','apply','--software-id','1','--dry-run','--json')
Invoke-CargoStrict @('run','--','cleanup','apply','--software-id','1','--confirm','--execution-ticket','phase8-ticket-cleanup-001','--json')

Write-Host '[phase8-smoke] download + ai + ui'
Invoke-CargoStrict @('run','--','repo','sync','--json')
Invoke-CargoStrict @('run','--','package','search','--json','--limit','3')
Invoke-CargoStrict @('run','--','download','start','--package-id','personal_local.sample','--dry-run','--json')
Invoke-CargoStrict @('run','--','ai','analyze','--json')
Invoke-CargoStrict @('run','--','ai','recommend','--goal','Rust development workstation','--json')
Invoke-CargoStrict @('run','--','ai','repair-plan','--software','PowerToys','--issue','crash on launch after update','--json')
Invoke-CargoStrict @('run','--','ui','search','--q','PowerToys','--json')
Invoke-CargoStrict @('run','--','ui','action-run','--id','software.show:111','--json')

Write-Host '[phase8-smoke] job queue basics'
$payloadFetch = '{\"package_id\":\"public_default.sample\"}'
$payloadVerify = '{\"job_id\":\"download-demo\"}'

Invoke-CargoStrict @('run','--','job','submit','--type','download.fetch','--payload',$payloadFetch,'--json')
$failedJobJson = (& cargo run --quiet -- job submit --type download.verify --payload $payloadVerify --simulate-failed --json)
if ($LASTEXITCODE -ne 0) {
    throw "cargo run -- job submit (simulate-failed) failed with exit code $LASTEXITCODE"
}
$failedJob = $failedJobJson | ConvertFrom-Json
if (-not $failedJob.job_id) {
    throw 'failed to parse job_id from simulate-failed submit output'
}
Invoke-CargoStrict @('run','--','job','list','--json','--limit','5')
Invoke-CargoStrict @('run','--','job','list','--json','--status','failed','--limit','5')
Invoke-CargoStrict @('run','--','job','retry','--id',"$($failedJob.job_id)",'--json')

Write-Host '[phase8-smoke] expected validation/security checks'
$expectedFailures = @(
    @{ args = @('run','--','update','apply','--candidate-id','180','--confirm','--json'); allowed = @(2) },
    @{ args = @('run','--','cleanup','apply','--software-id','1','--confirm','--json'); allowed = @(2) },
    @{ args = @('run','--','ui','search','--q','--json'); allowed = @(2) },
    @{ args = @('run','--','ui','action-run','--id','--json'); allowed = @(2) },
    @{ args = @('run','--','ui','action-run','--id','update.history:update-1771779929120392100-0-180','--json'); allowed = @(3) },
    @{ args = @('run','--','job','submit','--type','unknown.type','--payload','{}','--json'); allowed = @(2) },
    @{ args = @('run','--','job','list','--json','--status','unknown'); allowed = @(2) }
)

foreach ($item in $expectedFailures) {
    Write-Host "[phase8-smoke] expect-fail: cargo $($item.args -join ' ')"
    Invoke-CargoStrict -Args $item.args -AllowedExitCodes $item.allowed
    Write-Host '[phase8-smoke] expected failure captured'
}

Write-Host '[phase8-smoke] completed'
