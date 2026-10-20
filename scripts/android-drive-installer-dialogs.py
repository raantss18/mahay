#!/usr/bin/env python3
"""Answer Android's install dialogs on an emulator (tap Install/Update/Done) and report whether each
expected confirmation window appeared: used to test the hub's "Install all" queue.

    python3 -I scripts/android-drive-installer-dialogs.py SERIAL EXPECTED_DIALOGS TIMEOUT_SECONDS
"""
import re, subprocess, sys, time
adb = [subprocess.check_output("echo $HOME/Android/Sdk/platform-tools/adb", shell=True, text=True).strip(), "-s", sys.argv[1]]
def sh(*a, **k): return subprocess.run(adb + list(a), capture_output=True, text=True, **k).stdout
def dump():
    sh("shell", "uiautomator", "dump", "/sdcard/u.xml")
    return sh("shell", "cat", "/sdcard/u.xml")
def tap_text(xml, words):
    for m in re.finditer(r'<node[^>]*?text="([^"]*)"[^>]*?clickable="true"[^>]*?bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"', xml):
        t = m.group(1)
        if t in words:
            x = (int(m.group(2)) + int(m.group(4))) // 2; y = (int(m.group(3)) + int(m.group(5))) // 2
            sh("shell", "input", "tap", str(x), str(y)); return t
    return None
t0 = time.time(); last_event = t0; log = []
total = int(sys.argv[2]); seen_dialogs = 0
while time.time() - t0 < int(sys.argv[3]):
    xml = dump()
    pkg = re.findall(r'package="([^"]+)"', xml)
    top = pkg[0] if pkg else "?"
    hit = tap_text(xml, {"Install", "Update", "Done"}) if "packageinstaller" in top else None
    if hit:
        last_event = time.time(); log.append(f"{time.time()-t0:5.1f}s  system dialog ({top.split('.')[-1]}): tapped '{hit}'")
        if hit in ("Install", "Update"): seen_dialogs += 1
    else:
        stalled = time.time() - last_event
        if stalled > 20 and seen_dialogs < total:
            log.append(f"{time.time()-t0:5.1f}s  STALLED: no system dialog for {stalled:.0f}s, top window = {top}, confirm dialogs shown so far = {seen_dialogs}/{total}"); break
    if seen_dialogs >= total and not hit and time.time() - last_event > 12: break
    time.sleep(1.5)
print("\n".join(log)); print(f"confirm dialogs shown: {seen_dialogs}/{total}")
