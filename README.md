# ngm - Next Generation Package Manager

[![GitHub](https://img.shields.io/github/license/ngm-package-manager/ngm)](https://github.com/ngm-package-manager/ngm/blob/main/LICENSE)
[![CI Status](https://github.com/ngm-package-manager/ngm/workflows/CI/badge.svg)](https://github.com/ngm-package-manager/ngm/actions)
[![Documentation](https://img.shields.io/badge/docs-available-brightgreen)](https://ngm-package-manager.github.io/ngm/docs)

ngm is a next-generation package manager designed to aggregate multiple package repositories, resolve dependencies efficiently, and provide a seamless installation experience across macOS and Linux systems.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## Features

### Repository Aggregation
- Search across multiple package repositories simultaneously
- Supported repositories: PyPI, npm, Fedora, Ubuntu, RubyGems, and more
- No central package storage - directly source from official repositories

### Dependency Management
- Fast dependency tree resolution algorithm
- Pre-installation dependency visualization
- User confirmation before installing dependencies

### Installation Process
- Parallel downloads for maximum speed
- Progress tracking for all downloads
- Automatic PATH variable configuration
- Environment configuration management

## Installation

### Prerequisites
- Rust (for core components)
- Python 3.7+ (for CLI interface)
- Cargo (Rust package manager)
- pip (Python package manager)

### From Source

```bash
# Clone the repository
git clone https://github.com/ngm-package-manager/ngm.git
cd ngm

# Build the Rust components
cargo build --release

# Install Python dependencies
pip install -r python/requirements.txt

# Add ngm to your PATH
echo 'export PATH="$PATH:$PWD/rust/target/release"' >> ~/.bashrc
source ~/.bashrc