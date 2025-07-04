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
    command -v "$1" >/dev/null 2>&1
}

# Function to confirm action with user
confirm_action() {
    local message="$1"
    local default="${2:-n}"
    
    if [[ "$default" == "y" ]]; then
        read -rp "$message [Y/n] " response
        response=${response:-Y}
    else
        read -rp "$message [y/N] " response
        response=${response:-N}
    fi
    
    [[ $response =~ ^[Yy]$ ]]
}

# Function to safely remove file if it exists
safe_remove_file() {
    local file_path="$1"
    local description="$2"
    
    if [[ -f "$file_path" ]]; then
        if sudo rm -f "$file_path"; then
            success "Removed $description: $file_path"
        else
            warning "Failed to remove $description: $file_path"
            return 1
        fi
    else
        info "$description not found: $file_path"
    fi
    return 0
}

# Function to safely remove directory if it exists
safe_remove_directory() {
    local dir_path="$1"
    local description="$2"
    local force="${3:-false}"
    
    if [[ -d "$dir_path" ]]; then
        if [[ "$force" == "true" ]]; then
            if sudo rm -rf "$dir_path"; then
                success "Removed $description: $dir_path"
            else
                warning "Failed to remove $description: $dir_path"
                return 1
            fi
        else
            if sudo rmdir "$dir_path" 2>/dev/null; then
                success "Removed empty $description: $dir_path"
            else
                warning "$description is not empty, keeping: $dir_path"
                return 1
            fi
        fi
    else
        info "$description not found: $dir_path"
    fi
    return 0
}

# Constants matching install.sh
BINARY_NAME="teus"
BIN_DEST="/usr/local/bin"
CONF_DEST="/etc/systemd"
CONF_FILE="teus.toml"
SERVICE_FILE="teus.service"
SYSTEMD_DIR="/etc/systemd/system"
DATABASE_DIR="/var/lib/teus"
SERVICE_USER="teus"
SERVICE_GROUP="teus"

# Banner
echo -e "${BLUE}========================================"
echo -e "       TEUS Uninstall Script"
echo -e "========================================${NC}"

# Check for sudo privileges
if [[ $EUID -ne 0 && "$(id -u)" -ne 0 ]]; then
    if ! check_cmd sudo; then
        error "This script requires sudo privileges. Please run as root or install sudo."
    fi
fi

warning "This script will completely remove Teus and all its components from your system."
warning "This includes:"
echo -e "   • Systemd service (${SERVICE_FILE})"
echo -e "   • Binary executable (${BIN_DEST}/${BINARY_NAME})"
echo -e "   • Configuration file (${CONF_DEST}/${CONF_FILE})"
echo -e "   • Database and data directory (${DATABASE_DIR})"
echo -e "   • Service user and group (${SERVICE_USER})"
echo ""

if ! confirm_action "Are you sure you want to proceed with the uninstallation?"; then
    info "Uninstallation cancelled by user."
    exit 0
fi

echo ""
info "Starting Teus uninstallation..."

# Step 1: Stop and disable the systemd service
if check_cmd systemctl; then
    info "Managing systemd service..."
    
    # Check if service exists
    if systemctl list-unit-files | grep -q "^${SERVICE_FILE}"; then
        # Stop the service if it's running
        if systemctl is-active --quiet "${SERVICE_FILE}"; then
            info "Stopping Teus service..."
            if sudo systemctl stop "${SERVICE_FILE}"; then
                success "Teus service stopped."
            else
                warning "Failed to stop Teus service."
            fi
        else
            info "Teus service is not running."
        fi
        
        # Disable the service if it's enabled
        if systemctl is-enabled --quiet "${SERVICE_FILE}"; then
            info "Disabling Teus service..."
            if sudo systemctl disable "${SERVICE_FILE}"; then
                success "Teus service disabled."
            else
                warning "Failed to disable Teus service."
            fi
        else
            info "Teus service is not enabled."
        fi
    else
        info "Systemd service file not found."
    fi
else
    warning "systemctl not found. Skipping service management."
fi

# Step 2: Remove systemd service file
info "Removing systemd service file..."
safe_remove_file "${SYSTEMD_DIR}/${SERVICE_FILE}" "systemd service file"

# Step 3: Reload systemd daemon
if check_cmd systemctl; then
    info "Reloading systemd daemon..."
    if sudo systemctl daemon-reload; then
        success "Systemd daemon reloaded."
    else
        warning "Failed to reload systemd daemon."
    fi
fi

# Step 4: Remove the binary executable
info "Removing Teus binary..."
safe_remove_file "${BIN_DEST}/${BINARY_NAME}" "Teus binary executable"

# Step 5: Remove configuration file and backup
info "Removing configuration files..."
safe_remove_file "${CONF_DEST}/${CONF_FILE}" "configuration file"
safe_remove_file "${CONF_DEST}/${CONF_FILE}.bak" "configuration backup file"

# Step 6: Handle database directory
info "Handling database directory..."
if [[ -d "$DATABASE_DIR" ]]; then
    warning "Database directory found: $DATABASE_DIR"
    
    # Show directory contents if not empty
    if [[ -n "$(ls -A "$DATABASE_DIR" 2>/dev/null)" ]]; then
        echo "Directory contents:"
        ls -la "$DATABASE_DIR" 2>/dev/null || true
        echo ""
        
        if confirm_action "Do you want to remove the database directory and ALL data?"; then
            safe_remove_directory "$DATABASE_DIR" "database directory" "true"
        else
            warning "Database directory preserved with user data."
        fi
    else
        safe_remove_directory "$DATABASE_DIR" "empty database directory" "false"
    fi
else
    info "Database directory not found: $DATABASE_DIR"
fi

# Step 7: Remove service user and group
info "Removing service user and group..."

# Remove user if it exists
if id "$SERVICE_USER" >/dev/null 2>&1; then
    if sudo userdel "$SERVICE_USER"; then
        success "User '$SERVICE_USER' removed."
    else
        warning "Failed to remove user '$SERVICE_USER'."
    fi
else
    info "User '$SERVICE_USER' not found."
fi

# Remove group if it exists and is not used by other users
if getent group "$SERVICE_GROUP" >/dev/null 2>&1; then
    # Check if group has other members
    group_members=$(getent group "$SERVICE_GROUP" | cut -d: -f4)
    if [[ -z "$group_members" ]]; then
        if sudo groupdel "$SERVICE_GROUP"; then
            success "Group '$SERVICE_GROUP' removed."
        else
            warning "Failed to remove group '$SERVICE_GROUP'."
        fi
    else
        warning "Group '$SERVICE_GROUP' has other members, keeping it."
    fi
else
    info "Group '$SERVICE_GROUP' not found."
fi

# Step 8: Optional cleanup of build artifacts
if [[ -d "target" ]] && confirm_action "Remove build artifacts (target/ directory)?"; then
    if rm -rf target; then
        success "Build artifacts removed."
    else
        warning "Failed to remove build artifacts."
    fi
fi

# Step 9: Optional cleanup of development config
if [[ -f "teus-dev.toml" ]] && confirm_action "Remove development configuration file?"; then
    if rm -f "teus-dev.toml"; then
        success "Development configuration removed."
    else
        warning "Failed to remove development configuration."
    fi
fi

# Step 10: Information about what was NOT removed
echo ""
info "The following were NOT removed automatically:"
echo -e "   • Rust toolchain and Cargo (if installed by this script)"
echo -e "   • Diesel CLI (if installed by this script)"
echo -e "   • System dependencies (build tools, SQLite dev libraries)"
echo ""

if confirm_action "Do you want to remove Diesel CLI (installed via Cargo)?"; then
    if check_cmd cargo; then
        info "Removing Diesel CLI..."
        if cargo uninstall diesel_cli; then
            success "Diesel CLI removed."
        else
            warning "Failed to remove Diesel CLI or it was not installed via Cargo."
        fi
    else
        warning "Cargo not found. Cannot remove Diesel CLI automatically."
    fi
fi

echo ""
echo -e "${GREEN}=========================================="
echo -e "    Teus Uninstallation Complete!"
echo -e "==========================================${NC}"

success "Teus has been successfully removed from your system."

if [[ -d "$DATABASE_DIR" ]]; then
    warning "Note: Database directory was preserved at: $DATABASE_DIR"
    info "You can manually remove it later if needed: sudo rm -rf $DATABASE_DIR"
fi

info "If you installed Rust specifically for Teus, you can remove it with:"
echo -e "   ${BLUE}rustup self uninstall${NC}"
echo ""
info "Thank you for using Teus!" 