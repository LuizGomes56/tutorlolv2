param([ValidateSet('serve', 'build')][string]$Mode = 'serve', [switch]$PrintCommand)
$ErrorActionPreference = 'Stop'
$web = (Resolve-Path (Join-Path $PSScriptRoot '../../web')).ProviderPath
# Resolve CMD's temporary mapped drive back to its UNC share for WSL detection.
if ($web -match '^([A-Za-z]):\\') {
    $drive = Get-PSDrive -Name $Matches[1] -ErrorAction SilentlyContinue
    if ($drive.DisplayRoot) { $web = Join-Path $drive.DisplayRoot $web.Substring(3) }
}
$trunkArgs = @($Mode, '--features', 'tauri', '--minify', 'false')
if ($Mode -eq 'build') { $trunkArgs += '--release' }
else { $trunkArgs += @('--port', '8000') }

# A Windows Tauri build can use the WSL frontend without machine-specific paths.
if ($web -match '^\\\\wsl(?:\.localhost|\$)\\([^\\]+)\\(.*)$') {
    $distro = $Matches[1]
    $linuxWeb = '/' + $Matches[2].Replace('\', '/')
    $invokeArgs = @('--distribution', $distro, '--cd', $linuxWeb, '--exec', 'bash', '-lc', 'PATH="$HOME/.cargo/bin:$PATH" exec trunk "$@"', 'trunk') + $trunkArgs
    if ($PrintCommand) {
        @{ executable = 'wsl.exe'; arguments = $invokeArgs } | ConvertTo-Json -Depth 4
        exit 0
    }
    & wsl.exe @invokeArgs
} else {
    if ($PrintCommand) {
        @{ executable = 'trunk'; cwd = $web; arguments = $trunkArgs } | ConvertTo-Json -Depth 4
        exit 0
    }
    Push-Location $web
    try { & trunk @trunkArgs }
    finally { Pop-Location }
}
exit $LASTEXITCODE
