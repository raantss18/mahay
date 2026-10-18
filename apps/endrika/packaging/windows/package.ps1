<#
.SYNOPSIS
  Build, sign and package FigCraft for Windows.

.DESCRIPTION
  Produces, in $env:DIST (default: dist/release):
    figcraft-<version>-windows-<arch>.msi            per-machine installer (WiX v5)
    figcraft-<version>-windows-<arch>-portable.zip   figcraft.exe (+ figcraft-cli.exe) and the licences

  figcraft-cli.exe is built and shipped once apps/figcraft-cli exists in the workspace.

  The binaries link the C runtime statically (+crt-static), so neither the MSI nor the portable
  zip needs the Visual C++ redistributable. Signing is delegated to sign.ps1 (skipped with a
  warning when no signing secrets are set).

  Needs: Rust (MSVC toolchain + the target), the Windows SDK (rc.exe, signtool.exe),
  and WiX v5: dotnet tool install --global wix --version 5.0.2
             wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2

.EXAMPLE
  pwsh packaging/windows/package.ps1 -Arch x64
  pwsh packaging/windows/package.ps1 -Arch x86 -SkipBuild
  pwsh packaging/windows/package.ps1 -Arch arm64     # cross-compiled; needs the MSVC ARM64 build tools
#>
param(
  [ValidateSet('x64', 'x86', 'arm64')] [string] $Arch = 'x64',
  [switch] $SkipBuild
)
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path

function Invoke-Native([string] $What, [scriptblock] $Block) {
  Write-Output "==> $What"
  & $Block
  if ($LASTEXITCODE -ne 0) { throw "$What failed with exit code $LASTEXITCODE" }
}

# The version lives in one place: [workspace.package] version in the root Cargo.toml.
$Version = $env:FIGCRAFT_VERSION
if (-not $Version) {
  $inPkg = $false
  foreach ($line in Get-Content (Join-Path $Root 'Cargo.toml')) {
    if ($line -match '^\s*\[') { $inPkg = ($line.Trim() -eq '[workspace.package]'); continue }
    if ($inPkg -and $line -match '^\s*version\s*=\s*"([^"]+)"') { $Version = $Matches[1]; break }
  }
}
if (-not $Version) { throw 'could not read [workspace.package] version from Cargo.toml' }
# MSI ProductVersion is numeric (major.minor.build); pre-release tags are dropped there.
$MsiVersion = ($Version -split '-')[0]

$Target = switch ($Arch) { 'x64' { 'x86_64-pc-windows-msvc' } 'x86' { 'i686-pc-windows-msvc' } 'arm64' { 'aarch64-pc-windows-msvc' } }
$Dist = if ($env:DIST) { $env:DIST } else { Join-Path $Root 'dist\release' }
$TargetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $Root 'target' }
New-Item -ItemType Directory -Force -Path $Dist | Out-Null

if (-not $env:FIGCRAFT_BUILD_SHA) { $env:FIGCRAFT_BUILD_SHA = (git -C $Root rev-parse HEAD 2>$null) }
if (-not $env:FIGCRAFT_BUILD_DATE) { $env:FIGCRAFT_BUILD_DATE = (Get-Date).ToUniversalTime().ToString('yyyy-MM-dd') }

# The headless CLI ships next to the app once apps/figcraft-cli exists.
$HasCli = Test-Path (Join-Path $Root 'apps\figcraft-cli\Cargo.toml')
$Bins = @('figcraft'); if ($HasCli) { $Bins += 'figcraft-cli' }

Write-Output "FigCraft $Version for Windows $Arch ($Target): $($Bins -join ', ')"

if (-not $SkipBuild) {
  # Static CRT: no VC++ redistributable needed. Scoped to the target so host build scripts and
  # proc-macros are unaffected.
  $flagVar = 'CARGO_TARGET_' + ($Target.ToUpper() -replace '-', '_') + '_RUSTFLAGS'
  [Environment]::SetEnvironmentVariable($flagVar, '-C target-feature=+crt-static')
  $pkgs = @(); foreach ($b in $Bins) { $pkgs += @('-p', $b) }
  Invoke-Native "cargo build ($Target)" { cargo build --release --locked @pkgs --target $Target }
}

$Bin = Join-Path $TargetDir "$Target\release"

# Check both binaries' PE headers before packaging. Machine (COFF header) must match -Arch, so an
# x64 build can never ship labelled arm64. Subsystem (optional header): 2 = Windows GUI, 3 = console.
# The app must be GUI (no console window opens with it); the CLI must stay console so its output
# reaches the terminal.
function Get-PeHeader([string] $Path) {
  $bytes = [System.IO.File]::ReadAllBytes($Path)
  $pe = [BitConverter]::ToInt32($bytes, 0x3C)
  return @{ Machine = [BitConverter]::ToUInt16($bytes, $pe + 4); Subsystem = [BitConverter]::ToUInt16($bytes, $pe + 0x5C) }
}
$Machine = switch ($Arch) { 'x64' { 0x8664 } 'x86' { 0x14C } 'arm64' { 0xAA64 } }
$checks = @(, @('figcraft.exe', 2)); if ($HasCli) { $checks += , @('figcraft-cli.exe', 3) }
foreach ($check in $checks) {
  $h = Get-PeHeader (Join-Path $Bin $check[0])
  if ($h.Machine -ne $Machine) { throw "$($check[0]) is for machine 0x$('{0:X}' -f $h.Machine), expected 0x$('{0:X}' -f $Machine) ($Arch)" }
  if ($h.Subsystem -ne $check[1]) { throw "$($check[0]) has PE subsystem $($h.Subsystem), expected $($check[1])" }
  Write-Output "ok $($check[0]): $Arch, PE subsystem $($h.Subsystem)"
}
$Stage = Join-Path $TargetDir "windows-package\$Arch"
Remove-Item -Recurse -Force $Stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Stage | Out-Null
$Exes = $Bins | ForEach-Object { "$_.exe" }
foreach ($exe in $Exes) { Copy-Item (Join-Path $Bin $exe) $Stage }

& (Join-Path $PSScriptRoot 'sign.ps1') @($Exes | ForEach-Object { Join-Path $Stage $_ })

# ---- MSI ---------------------------------------------------------------------------------------
# The setup wizard's art, drawn here from the app icon so the MSI shows FigCraft rather than
# WiX's stock bitmaps: a 493x58 banner across the top of the inner pages (the page title is drawn
# over its left side, so that stays white) and a 493x312 backdrop for the Welcome and Finish pages
# (their text sits on the right; the left 164 px are ours). Both must be 24-bit BMPs.
function New-InstallerArt([string] $IconPng, [string] $BannerOut, [string] $DialogOut) {
  Add-Type -AssemblyName System.Drawing
  $paper = [System.Drawing.ColorTranslator]::FromHtml('#f5f5f5')
  $accent = [System.Drawing.ColorTranslator]::FromHtml('#0d99ff')
  $icon = [System.Drawing.Image]::FromFile($IconPng)
  try {
    foreach ($spec in @(@($BannerOut, 493, 58), @($DialogOut, 493, 312))) {
      $bmp = New-Object System.Drawing.Bitmap $spec[1], $spec[2], ([System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
      $g = [System.Drawing.Graphics]::FromImage($bmp)
      try {
        $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $g.Clear([System.Drawing.Color]::White)
        if ($spec[2] -eq 58) {
          $g.DrawImage($icon, 493 - 46 - 8, 6, 46, 46)
        } else {
          $panel = New-Object System.Drawing.SolidBrush $paper
          $stripe = New-Object System.Drawing.SolidBrush $accent
          $g.FillRectangle($panel, 0, 0, 164, 312)
          $g.FillRectangle($stripe, 160, 0, 4, 312)
          $g.DrawImage($icon, 18, 64, 124, 124)
          $panel.Dispose(); $stripe.Dispose()
        }
      } finally { $g.Dispose() }
      $bmp.Save($spec[0], [System.Drawing.Imaging.ImageFormat]::Bmp)
      $bmp.Dispose()
    }
  } finally { $icon.Dispose() }
}
$Banner = Join-Path $Stage 'installer-banner.bmp'
$Dialog = Join-Path $Stage 'installer-dialog.bmp'
New-InstallerArt (Join-Path $Root 'assets\app-icon\hicolor\256x256\apps\ai.storyteller.figcraft.png') $Banner $Dialog

$Msi = Join-Path $Dist "figcraft-$Version-windows-$Arch.msi"
& (Join-Path $PSScriptRoot 'check-icons.ps1')
Invoke-Native 'wix build' {
  wix build (Join-Path $PSScriptRoot 'figcraft.wxs') -arch $Arch `
    -ext WixToolset.UI.wixext -ext WixToolset.Util.wixext `
    -culture en-US -loc (Join-Path $PSScriptRoot 'figcraft.en-us.wxl') `
    -d "Version=$MsiVersion" -d "DisplayVersion=$Version" -d "BinDir=$Stage" -d "IconPath=$(Join-Path $Root 'assets\app-icon\figcraft.ico')" `
    -d "BannerBmp=$Banner" -d "DialogBmp=$Dialog" -d "HasCli=$([int]$HasCli)" `
    -o $Msi
}
Invoke-Native 'MSI shortcut icon validation (ICE50)' {
  wix msi validate $Msi -ice ICE50 -intermediateFolder (Join-Path $Stage 'msi-validation')
}
# wix writes its debug symbols (.wixpdb) next to the MSI; keep them out of the release assets.
Remove-Item -Force -ErrorAction SilentlyContinue ([IO.Path]::ChangeExtension($Msi, '.wixpdb'))
& (Join-Path $PSScriptRoot 'sign.ps1') $Msi

# ---- portable zip ------------------------------------------------------------------------------
$Portable = Join-Path $TargetDir "windows-package\figcraft-$Version-windows-$Arch-portable"
Remove-Item -Recurse -Force $Portable -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Portable | Out-Null
Copy-Item (Join-Path $Stage '*.exe') $Portable
foreach ($f in 'README.md', 'LICENSE', 'LICENSE-MIT', 'LICENSE-APACHE', 'NOTICE') {
  $p = Join-Path $Root $f
  if (Test-Path $p) { Copy-Item $p $Portable }
}
# Builds made with craft-fonts (CRAFT_FONTS_DIR, all official releases) embed its OFL-1.1 fonts:
# ship each font's licence as OFL-<family-dir>.txt.
if ($env:CRAFT_FONTS_DIR) {
  Get-ChildItem -Path (Join-Path $env:CRAFT_FONTS_DIR 'fonts') -Directory -ErrorAction SilentlyContinue | ForEach-Object {
    $lic = Join-Path $_.FullName 'OFL.txt'
    if (Test-Path $lic) { Copy-Item $lic (Join-Path $Portable "OFL-$($_.Name).txt") }
  }
}
# files go to FigCraftData\ next to the exe instead of %APPDATA% (#228; see app_dirs.rs).
$Zip = Join-Path $Dist "figcraft-$Version-windows-$Arch-portable.zip"
Remove-Item -Force $Zip -ErrorAction SilentlyContinue
Compress-Archive -Path $Portable -DestinationPath $Zip

# Smoke-test the CLI when this machine can run it. An ARM64 build made on an x64 runner can't run
# here; .github/workflows/windows-arm64.yml installs and runs it on ARM64 instead.
$HostArch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
$Smoke = if ($HasCli) { 'figcraft-cli.exe' } else { 'figcraft.exe' }
if ($Arch -ne 'arm64' -or $HostArch -eq 'arm64') {
  # figcraft.exe is a GUI-subsystem exe: pipe it so PowerShell waits and shows its --version output.
  Invoke-Native "$Smoke --version" { & (Join-Path $Stage $Smoke) --version | Out-Host }
} else {
  Write-Output "skipping $Smoke --version: an $Arch build doesn't run on this $HostArch machine"
}
Get-Item $Msi, $Zip | Format-Table Name, Length
