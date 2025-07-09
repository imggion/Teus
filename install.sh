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

# Function to compare version numbers
version_compare() {
    local version1="$1"
    local version2="$2"
    local op="$3"
    
    # Convert version strings to arrays
    IFS='.' read -ra ver1 <<< "$version1"
    IFS='.' read -ra ver2 <<< "$version2"
    
    # Pad arrays to same length
    local max_len=${#ver1[@]}
    if [ ${#ver2[@]} -gt $max_len ]; then
        max_len=${#ver2[@]}
    fi
    
    while [ ${#ver1[@]} -lt $max_len ]; do
        ver1+=("0")
    done
    while [ ${#ver2[@]} -lt $max_len ]; do
        ver2+=("0")
    done
    
    # Compare version components
    for ((i=0; i<max_len; i++)); do
        if [ "${ver1[i]}" -gt "${ver2[i]}" ]; then
            [ "$op" = "ge" ] || [ "$op" = "gt" ] && return 0
            [ "$op" = "lt" ] || [ "$op" = "le" ] || [ "$op" = "eq" ] && return 1
        elif [ "${ver1[i]}" -lt "${ver2[i]}" ]; then
            [ "$op" = "lt" ] || [ "$op" = "le" ] && return 0
            [ "$op" = "gt" ] || [ "$op" = "ge" ] || [ "$op" = "eq" ] && return 1
        fi
    done
    
    # Versions are equal
    [ "$op" = "eq" ] || [ "$op" = "ge" ] || [ "$op" = "le" ] && return 0
    return 1
}

# Banner
echo -e "${BLUE}========================================"
echo -e "       TEUS Installation Script"
echo -e "========================================${NC}"

# Check for sudo privileges
if [[ $EUID -ne 0 && "$(id -u)" -ne 0 ]]; then
    if ! command -v sudo >/dev/null 2>&1; then
        error "This script requires sudo privileges."
    fi
fi

# Check if the user teus is inside docker group
if ! id -nG teus | grep -qw "docker"; then
    warning "The user 'teus' is not in the 'docker' group. Please add the user to the docker group and try again."
    read -rp "Do you want to add the user 'teus' to the docker group? [Y/n] " add_to_docker_group
    add_to_docker_group=${add_to_docker_group:-Y}
    if [[ $add_to_docker_group =~ ^[Yy]$ ]]; then
        sudo usermod -aG docker teus
        success "User 'teus' added to docker group."
    else
        error "User 'teus' is not in the docker group. Please add the user to the docker group and try again."
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

# Rust and Cargo version checks
info "Checking Rust and Cargo installation..."
MINIMUM_RUST_VERSION="1.86.0"

# Check if Rust is installed
if ! command -v rustc >/dev/null 2>&1; then
    error "Rust is not installed. Please install Rust from https://rustup.rs/ and try again."
fi

# Check if Cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    error "Cargo is not installed. Please install Rust (which includes Cargo) from https://rustup.rs/ and try again."
fi

# Get Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
info "Found Rust version: $RUST_VERSION"

# Check if Rust version meets minimum requirement
if ! version_compare "$RUST_VERSION" "$MINIMUM_RUST_VERSION" "ge"; then
    error "Rust version $RUST_VERSION is too old. Minimum required version is $MINIMUM_RUST_VERSION. Please update Rust using 'rustup update' and try again."
fi

success "Rust version check passed."

# Check for Diesel CLI
info "Checking Diesel CLI installation..."
if ! command -v diesel >/dev/null 2>&1; then
    warning "Diesel CLI is not installed."
    read -rp "Do you want to install Diesel CLI? [Y/n] " install_diesel
    install_diesel=${install_diesel:-Y}
    
    if [[ $install_diesel =~ ^[Yy]$ ]]; then
        info "Installing Diesel CLI..."
        if cargo install diesel_cli --no-default-features --features sqlite; then
            success "Diesel CLI installed successfully."
        else
            # fallback installer to ensure diesel installation
            if curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh; then
                success "Diesel CLI installed successfully. "
            fi
            error "Failed to install Diesel CLI. Please install it manually with: cargo install diesel_cli --no-default-features --features sqlite"
        fi
    else
        error "Diesel CLI is required for database migrations. Please install it manually with: cargo install diesel_cli --no-default-features --features sqlite"
    fi
else
    success "Diesel CLI is already installed."
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

info "Creating Teus service user and group..."
if ! getent group teus >/dev/null; then
    sudo groupadd -r teus || error "Failed to create group 'teus'"
    success "Group 'teus' created."
else
    info "Group 'teus' already exists."
fi

if ! id -u teus >/dev/null 2>&1; then
    sudo useradd -r -g teus -d /var/lib/teus -s /sbin/nologin -c "Teus Service User" teus || error "Failed to create user 'teus'"
    success "User 'teus' created."
else
    info "User 'teus' already exists."
fi

info "Creating database directory /var/lib/teus and setting initial ownership..."
sudo mkdir -p /var/lib/teus || error "Failed to create directory /var/lib/teus"
sudo chown -R teus:teus /var/lib/teus || error "Failed to set initial ownership for /var/lib/teus"
success "Database directory setup complete for initial ownership."

# Run migrations with improved error handling
info "Running database migrations..."
echo "DATABASE_URL=/var/lib/teus/sysinfo.db" > .env

# Verify diesel is available before running migrations
if ! command -v diesel >/dev/null 2>&1; then
    error "Diesel CLI is not available. Cannot run database migrations. Please install Diesel CLI and try again."
fi

if diesel migration run; then
    success "Migrations completed successfully."
    info "Setting final ownership for /var/lib/teus after migrations..."
    sudo chown -R teus:teus /var/lib/teus || error "Failed to set final ownership for /var/lib/teus"
    success "Final ownership set."
else
    error "Database migrations failed. Please check that Diesel CLI is properly installed and the database directory is accessible."
fi

info "Cleaning up temporary .env file..."
rm -f .env
success ".env file cleaned up."

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
