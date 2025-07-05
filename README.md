# Teus

[![Rust Teus CI](https://github.com/imggion/Teus/actions/workflows/rust.yml/badge.svg)](https://github.com/imggion/Teus/actions/workflows/rust.yml)

Teus is a lightweight system monitoring service written in Rust that collects and exposes system metrics through a REST API.

## Features

- Real-time system metrics collection (CPU, RAM, swap, disk usage)
- Persistent storage of metrics in SQLite database
- RESTful API for accessing historical and current system data
- Configurable monitoring intervals
- Runs as a system service

## Installation

You can install Teus using the provided installation script:

```bash
# Clone the repository
git clone https://github.com/yourusername/teus.git
cd teus

# Run the installation script
sudo ./install.sh
```

The installation script will:
- Build the Rust project
- Install the binary to /usr/local/bin
- Set up the configuration file in /etc/systemd/
- Create the necessary database directory
- Register and start the systemd service

## Configuration

Teus is configured through a TOML file (default: `/etc/systemd/teus.toml`):

```toml
[server]
host = "127.0.0.1"
port = 26783

[database]
path = "/var/lib/teus/sysinfo.db"

[monitor]
interval_secs = 5
```

You can specify a custom configuration file path when running Teus:

```bash
teus /path/to/your/config.toml
```

## Usage

Once installed, Teus runs as a system service and automatically starts on boot:

```bash
# Check service status
sudo systemctl status teus

# Manually start the service
sudo systemctl start teus

# Stop the service
sudo systemctl stop teus

# Restart the service
sudo systemctl restart teus
```

## API Endpoints

Teus provides a RESTful API to access system metrics:

- `GET /api/sysinfo` - Get the latest system metrics

## Project Structure

## Requirements

- Rust 2024 edition or later
- SQLite
- Linux-based operating system (for full system metrics support)
- System dependencies:
  - `build-essential` - Essential compilation tools
  - `libsqlite3-dev` - SQLite development libraries

The installation script (`install.sh`) automatically checks for these dependencies and installs them if they're missing (debian based).

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Created by [gdjohn4s](https://github.com/gdjohn4s)
