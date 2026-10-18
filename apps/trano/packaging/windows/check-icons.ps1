<#
.SYNOPSIS
  Check MSI icon references without building or installing ArtCraft Launcher.

.DESCRIPTION
  Windows Installer caches each <Icon> under its Id as a file name, so a shortcut icon Id needs an
  .exe or .ico extension (ICE50 for advertised shortcuts; extensionless identifiers can display a
  generic document icon even when the ICO data and application are valid). Also checks that every
  shortcut and ARPPRODUCTICON reference an <Icon> that exists, and that the Start Menu shortcut
  carries the app's AppUserModelID (toast notifications need it).
#>
param(
  [string] $WxsPath = (Join-Path $PSScriptRoot 'artcraft-launcher.wxs')
)
$ErrorActionPreference = 'Stop'
$AppId = 'ai.storyteller.artcraft-launcher'
[xml] $Wxs = Get-Content -LiteralPath $WxsPath -Raw
$Ns = [Xml.XmlNamespaceManager]::new($Wxs.NameTable)
$Ns.AddNamespace('w', 'http://wixtoolset.org/schemas/v4/wxs')
$Icons = @{}
foreach ($icon in $Wxs.SelectNodes('//w:Icon', $Ns)) {
  $Icons[$icon.GetAttribute('Id')] = $icon
}

$shortcuts = $Wxs.SelectNodes('//w:Shortcut', $Ns)
if ($shortcuts.Count -eq 0) { throw 'No <Shortcut> in the MSI.' }
foreach ($shortcut in $shortcuts) {
  $id = $shortcut.GetAttribute('Id')
  $iconId = $shortcut.GetAttribute('Icon')
  if (-not $Icons.ContainsKey($iconId)) {
    throw "Shortcut '$id' references missing icon '$iconId'."
  }
  $iconExtension = [IO.Path]::GetExtension($iconId)
  if ($iconExtension -notin '.exe', '.ico') {
    throw "Shortcut '$id' icon '$iconId' must have an .exe or .ico extension (ICE50)."
  }
  $aumid = $shortcut.SelectSingleNode('w:ShortcutProperty[@Key="System.AppUserModel.ID"]', $Ns)
  if (-not $aumid -or $aumid.GetAttribute('Value') -ne $AppId) {
    throw "Shortcut '$id' must set System.AppUserModel.ID to $AppId."
  }
}

$arpIcon = $Wxs.SelectSingleNode('//w:Property[@Id="ARPPRODUCTICON"]', $Ns)
if ($arpIcon -and -not $Icons.ContainsKey($arpIcon.GetAttribute('Value'))) {
  throw "ARPPRODUCTICON references missing icon '$($arpIcon.GetAttribute('Value'))'."
}
Write-Output 'Windows shortcut icon references ok'
