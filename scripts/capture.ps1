# Run in a logged-in Windows desktop session, not an SSH-only session.
# Example: .\scripts\capture.ps1 -Target windows-winui
param(
    [ValidateSet('windows-winui','windows-xaml')]
    [string]$Target = 'windows-winui',
    [string]$Cli = '',
    [switch]$SmallBuild
)
$ErrorActionPreference = 'Continue' # Native tools write progress to stderr.
Set-Location (Join-Path $PSScriptRoot '..')
if (!$Cli) { $Cli = if ($env:DAY_CLI) { $env:DAY_CLI } else { 'day' } }
if ($SmallBuild) {
    $env:CARGO_PROFILE_DEV_DEBUG = '0'
    $env:CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG = '0'
    $env:CARGO_PROFILE_DEV_INCREMENTAL = 'false'
    $env:CARGO_INCREMENTAL = '0'
}
& $Cli launch -t $Target --script dayscript/capture-desktop.yaml `
    --locales 'en,fr,ar,de,ja,zh-CN,ko' --themes 'light,dark'
exit $LASTEXITCODE
