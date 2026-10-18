#!/usr/bin/env bash
# App-specific installer and package assertions retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

for want in '<UIRef Id="HazavanaUI" />' 'Value="InstallDirDlg"' '<Property Id="WIXUI_INSTALLDIR" Value="INSTALLFOLDER" />' \
    'Key="Software\Hazavana" Name="InstallFolder"' 'Value="LaunchHazavana"'; do
  grep -qF "$want" packaging/windows/hazavana.wxs || { echo "::error::hazavana.wxs lost: $want"; exit 1; }
done
grep -qF '<DialogRef Id="ExitDialog" />' packaging/windows/hazavana.wxs \
  || { echo "::error::ExitDialog is not referenced; installs and removes end without a completion dialog"; exit 1; }

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("org.mahay.hazavana.yml", "org.mahay.hazavana.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "org.mahay.hazavana" and manifest["command"] == "hazavana", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("Hazavana package identity ok")
PYEOF
