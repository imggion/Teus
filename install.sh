#!/bin/bash
set -e

MISSING=""
if ! command -v cc >/dev/null 2>&1; then
    MISSING+=" build-essential"
fi

if ! dpkg -s libsqlite3-dev >/dev/null 2>&1; then
    MISSING+=" libsqlite3-dev"
fi

if [ -n "$MISSING" ]; then
    echo "Following packages are not installed: $MISSING"
    echo "Installing..."
    sleep 2
    sudo apt-get update && sudo apt-get install -y $MISSING
fi

# Name of the generated executable (modify if necessary)
BINARY_NAME="teus"

# Build directory for release mode
BUILD_DIR="target/release"

# Destination directory for the executable
BIN_DEST="/usr/local/bin"

# Destination directory for the configuration file
CONF_DEST="/etc/systemd"
CONF_FILE="teus.toml"

# Name of the systemd unit file (ensure teus.service is in the current directory)
SERVICE_FILE="teus.service"
SYSTEMD_DIR="/etc/systemd/system"

echo "=========================================="
echo "Building the Rust project in release mode..."
cargo build --release

echo "=========================================="
echo "Copying the executable ${BINARY_NAME} to ${BIN_DEST}..."
if [ -f "${BIN_DEST}/${BINARY_NAME}" ]; then
    sudo rm -f "${BIN_DEST}/${BINARY_NAME}"
fi
sudo cp "${BUILD_DIR}/${BINARY_NAME}" "${BIN_DEST}/${BINARY_NAME}"
sudo chmod +x "${BIN_DEST}/${BINARY_NAME}"

echo "=========================================="
echo "Copying the configuration file ${CONF_FILE} to ${CONF_DEST}..."
if [ -f "${CONF_DEST}/${CONF_FILE}" ]; then
    sudo rm -f "${CONF_DEST}/${CONF_FILE}"
fi
sudo cp "${CONF_FILE}" "${CONF_DEST}/${CONF_FILE}"

echo "=========================================="
echo "Copying the service file ${SERVICE_FILE} to ${SYSTEMD_DIR}..."
if [ -f "${SYSTEMD_DIR}/${SERVICE_FILE}" ]; then
    sudo rm -f "${SYSTEMD_DIR}/${SERVICE_FILE}"
fi
sudo cp "${SERVICE_FILE}" "${SYSTEMD_DIR}/${SERVICE_FILE}"

echo "=========================================="
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

echo "=========================================="
echo "Enabling the service ${SERVICE_FILE}..."
sudo systemctl enable "${SERVICE_FILE}"

echo "=========================================="
echo "Starting the service ${SERVICE_FILE}..."
sudo systemctl start "${SERVICE_FILE}"

echo "=========================================="
echo "Installation completed successfully!"
echo "You can now start the service with: systemctl start ${SERVICE_FILE}"
echo "Or check the status with: systemctl status ${SERVICE_FILE}"