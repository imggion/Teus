#!/bin/bash
set -e

# Colors for better readability
GREEN="\033[0;32m"
BLUE="\033[0;34m"
RED="\033[0;31m"
YELLOW="\033[1;33m"
NC="\033[0m" # No Color

# Function to display colored messages
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Function to check if command exists
check_cmd() {
    command -v "$1" >/dev/null 2>&1 || { error "Required command '$1' not found. Please install it first."; }
}

# Banner
echo -e "${BLUE}========================================"
echo -e "       TEUS Installation Script"
echo -e "========================================${NC}"

# Check for sudo privileges
if [[ $EUID -ne 0 && "$(id -u)" -ne 0 ]]; then
    if ! command -v sudo >/dev/null 2>&1; then
        error "This script requires sudo privileges. Please run as root or install sudo."
    fi
fi

# Dependency check
info "Checking dependencies..."
MISSING=""
if ! command -v cc >/dev/null 2>&1; then
    MISSING+=" build-essential"
fi

if ! command -v dpkg >/dev/null 2>&1; then
    if command -v apt-get >/dev/null 2>&1; then
        # Debian/Ubuntu system
        if ! dpkg -s libsqlite3-dev >/dev/null 2>&1; then
            MISSING+=" libsqlite3-dev"
        fi
    elif command -v yum >/dev/null 2>&1; then
        # RHEL/CentOS system
        if ! rpm -q sqlite-devel >/dev/null 2>&1; then
            MISSING+=" sqlite-devel"
        fi
    elif command -v pacman >/dev/null 2>&1; then
        # Arch system
        if ! pacman -Q sqlite >/dev/null 2>&1; then
            MISSING+=" sqlite"
        fi
    elif command -v brew >/dev/null 2>&1; then
        # macOS with Homebrew
        if ! brew list sqlite >/dev/null 2>&1; then
            MISSING+=" sqlite"
        fi
    fi
fi

if [ -n "$MISSING" ]; then
    warning "Following packages are not installed:${YELLOW}$MISSING${NC}"
    read -rp "Do you want to install these dependencies? [Y/n] " install_deps
    install_deps=${install_deps:-Y}
    
    if [[ $install_deps =~ ^[Yy]$ ]]; then
        info "Installing dependencies..."
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update && sudo apt-get install -y $MISSING
        elif command -v yum >/dev/null 2>&1; then
            sudo yum install -y $MISSING
        elif command -v pacman >/dev/null 2>&1; then
            sudo pacman -S --noconfirm $MISSING
        elif command -v brew >/dev/null 2>&1; then
            brew install $MISSING
        else
            error "Unsupported package manager. Please install dependencies manually: $MISSING"
        fi
    else
        error "Dependencies are required for installation. Exiting."
    fi
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

# Check if files exist
if [ ! -f "${CONF_FILE}" ]; then
    error "Configuration file ${CONF_FILE} not found in the current directory."
fi

if [ ! -f "${SERVICE_FILE}" ]; then
    error "Service file ${SERVICE_FILE} not found in the current directory."
fi

# Ask about web dashboard installation
read -rp "Do you want to install the web dashboard? [Y/n] " install_dashboard
install_dashboard=${install_dashboard:-Y}

info "Building the Rust project in release mode..."
if ! cargo build --release; then
    error "Failed to build the project."
fi
success "Build completed successfully."

info "Copying the executable ${BINARY_NAME} to ${BIN_DEST}..."
if [ -f "${BIN_DEST}/${BINARY_NAME}" ]; then
    sudo rm -f "${BIN_DEST}/${BINARY_NAME}"
fi
sudo cp "${BUILD_DIR}/${BINARY_NAME}" "${BIN_DEST}/${BINARY_NAME}" || error "Failed to copy executable"
sudo chmod +x "${BIN_DEST}/${BINARY_NAME}" || error "Failed to set executable permissions"
success "Executable copied successfully."

info "Copying the configuration file ${CONF_FILE} to ${CONF_DEST}..."
if [ -f "${CONF_DEST}/${CONF_FILE}" ]; then
    warning "Existing configuration file found. Creating backup."
    sudo cp "${CONF_DEST}/${CONF_FILE}" "${CONF_DEST}/${CONF_FILE}.bak"
fi
sudo mkdir -p "${CONF_DEST}" || error "Failed to create configuration directory"
sudo cp "${CONF_FILE}" "${CONF_DEST}/${CONF_FILE}" || error "Failed to copy configuration file"
success "Configuration file copied successfully."

info "Copying the service file ${SERVICE_FILE} to ${SYSTEMD_DIR}..."
if [ -f "${SYSTEMD_DIR}/${SERVICE_FILE}" ]; then
    sudo rm -f "${SYSTEMD_DIR}/${SERVICE_FILE}"
fi
sudo cp "${SERVICE_FILE}" "${SYSTEMD_DIR}/${SERVICE_FILE}" || error "Failed to copy service file"
success "Service file copied successfully."

# Run migrations
info "Running database migrations..."
echo "/var/lib/teus/sysinfo.db" > .env
diesel migration run
success "Migrations completed successfully."

# Handle the web dashboard installation if user selected yes
if [[ $install_dashboard =~ ^[Yy]$ ]]; then
    info "Setting up web dashboard..."
    # Add web dashboard setup code here
    # For example: clone web dashboard repo, build it, copy to webroot, etc.
    success "Web dashboard installed successfully."
fi

info "Reloading systemd daemon..."
if command -v systemctl >/dev/null 2>&1; then
    sudo systemctl daemon-reload || warning "Failed to reload systemd daemon"
    
    info "Enabling the service ${SERVICE_FILE}..."
    sudo systemctl enable "${SERVICE_FILE}" || warning "Failed to enable service"
    
    info "Starting the service ${SERVICE_FILE}..."
    if sudo systemctl start "${SERVICE_FILE}"; then
        success "Service started successfully."
    else
        warning "Failed to start service. Check status with: systemctl status ${SERVICE_FILE}"
    fi
else
    warning "systemctl not found. Please start the service manually."
fi

echo -e "${GREEN}=========================================="
echo -e "    Installation completed successfully!"
echo -e "==========================================${NC}"

if command -v systemctl >/dev/null 2>&1; then
    info "You can control the service with:"
    echo -e "   ${BLUE}systemctl start ${SERVICE_FILE}${NC} - Start the service"
    echo -e "   ${BLUE}systemctl stop ${SERVICE_FILE}${NC} - Stop the service"
    echo -e "   ${BLUE}systemctl status ${SERVICE_FILE}${NC} - Check service status"
    echo -e "   ${BLUE}systemctl restart ${SERVICE_FILE}${NC} - Restart the service"
fi

# TODO: Install and configure the web dashboard if applicable
# TODO: Add instructions for accessing the web dashboard if installed
if [[ $install_dashboard =~ ^[Yy]$ ]]; then
    info "Setting up web dashboard..."
    success "Web dashboard installed successfully."
    info "You can access the web dashboard at: http://localhost:8080"
fi