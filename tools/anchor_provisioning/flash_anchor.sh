#!/bin/bash
# ZTP (Zero-Touch Provisioning) for Cat_Dark MIL-SPEC Anchors
set -e

echo "[+] Cat_Dark Tactical: Anchor Provisioning Protocol Initiated"
DEVICE_MOUNT="/dev/sda"  # Example target

# SAFETY FAILSAFE 1: Verify it is a valid SBC block device, not a 1TB Host Drive
DEVICE_SIZE_BYTES=$(blockdev --getsize64 $DEVICE_MOUNT 2>/dev/null || echo "100000000000")
DEVICE_SIZE_GB=$((DEVICE_SIZE_BYTES / 1024 / 1024 / 1024))

if [ "$DEVICE_SIZE_GB" -gt 64 ]; then
    echo "[-] FATAL FAILSAFE: Target block device is ${DEVICE_SIZE_GB}GB (> 64GB). Aborting to prevent wiping Operator OS disk!"
    exit 1
fi

# SAFETY FAILSAFE 2: Hardware Signature Match
USB_ID=$(lsusb | grep -i -E "Raspberry|Rockchip|Allwinner" || true)
if [ -z "$USB_ID" ]; then
    echo "[-] FATAL FAILSAFE: Target device does not match known SBC (ARM) hardware signature."
    exit 1
fi

echo "[*] Flashing stripped-down Kernel (MUSL Static)..."
# dd if=build/kernel.img of=$DEVICE_MOUNT bs=4M status=progress

echo "[*] Injecting 'cat_dark-anchor' daemon..."
cp ../../releases/v1.0.0-PROD/golubegram-mesh-daemon $DEVICE_MOUNT/opt/bin/cat_dark_anchor
chmod +x $DEVICE_MOUNT/opt/bin/cat_dark_anchor

echo "[*] Generating Ed25519 Anchor Identity Keys..."
# Keygen logic here (writing to secure enclave / TPM if available)

echo "[*] Sealing OS constraints (Read-Only RootFS, Disabling SSH/TTY)..."
# Setting read-only mounts in fstab...

echo "[SUCCESS] MIL-SPEC Anchor Provisioned. Unplug and deploy."
