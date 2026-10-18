---
title: Install Pejy
description: Download Pejy 0.7.1 for Windows 11, macOS or Linux, or install it from Mahay Trano.
order: 20
verified: 0.7.1
---

Download the file for your system from the [latest release](https://github.com/raantss18/mahay/releases/latest) and install it. In the file names, `<version>` stands for the version number, for example `0.7.1`. The release also has `SHA256SUMS.txt`, which lists a checksum for every file. The [Pejy page on mahay.com](https://github.com/raantss18/mahay) links to the download as well.

## Windows 11

1. Open the [latest release](https://github.com/raantss18/mahay/releases/latest).
2. Download `pejy-<version>-windows-x64.msi`. It fits most PCs (64-bit Intel and AMD).
3. Run the file and follow the installer.
4. Start **Pejy** from the Start menu.

The release also has an `arm64` installer for ARM PCs such as Snapdragon, and a 32-bit `x86` installer. To run without installing, download `pejy-<version>-windows-x64-portable.zip` (or the `arm64` or `x86` version), unzip it and start the program inside. The installers and executables are code-signed.

## macOS

1. Download `pejy-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs.
2. Open it and drag Pejy to Applications.

The app is signed and notarized.

## Linux

Pick one file from the release. Replace `x86_64` with `aarch64` on ARM.

| File                                          | Notes                                                    |
|-----------------------------------------------|----------------------------------------------------------|
| `pejy-<version>-linux-x86_64.AppImage` | Runs anywhere.                                           |
| `pejy-<version>-linux-x86_64.flatpak`  | Sandboxed. Install with `flatpak install --user <file>`. |
| `pejy-<version>-linux-x86_64.deb`      | Debian and Ubuntu.                                       |
| `pejy-<version>-linux-x86_64.rpm`      | Fedora, RHEL and openSUSE.                               |
| `pejy-<version>-linux-x86_64.tar.gz`   | Unpack anywhere.                                         |

For RISC-V there is a `.deb`, an `.rpm` and a tarball, named with `riscv64` in place of `x86_64`. There is no AppImage or Flatpak for RISC-V. The RISC-V build needs glibc 2.39 or newer (Ubuntu 24.04 or newer). A FreeBSD build (`pejy-<version>-freebsd-x86_64.tar.gz`) is on the release page too.

## Install with Mahay Trano

Mahay Trano lists Pejy and can install and update it for you. See [Mahay Trano](https://github.com/raantss18/mahay) for how it works.

## First run

Open Pejy. You land on the start screen. Continue with [Getting started](index.md). If the window closes right away, see [Pejy closes right after it starts](../troubleshooting/app-closes-at-start.md).
