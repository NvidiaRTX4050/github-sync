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

# Copy systemd service file
cp github-sync.service /etc/systemd/system/github-sync@.service

# Reload systemd daemon
systemctl daemon-reload

# Enable and start the service for the user
systemctl enable github-sync@${REAL_USER}
systemctl start github-sync@${REAL_USER}

echo "✅ GitHub Sync service installed and enabled for user: ${REAL_USER}"
echo "ℹ️  Service status:"
systemctl status github-sync@${REAL_USER} 