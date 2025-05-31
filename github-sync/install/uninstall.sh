#!/bin/bash

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root"
    exit 1
fi

# Get the username of the user who ran sudo
REAL_USER=$(logname || echo $SUDO_USER)
if [ -z "$REAL_USER" ]; then
    echo "Could not determine the real user"
    exit 1
fi

# Stop and disable the service
systemctl stop github-sync@${REAL_USER}
systemctl disable github-sync@${REAL_USER}

# Remove service file
rm -f /etc/systemd/system/github-sync@.service

# Reload systemd daemon
systemctl daemon-reload

echo "✅ GitHub Sync service uninstalled for user: ${REAL_USER}" 