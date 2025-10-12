# CCometixLine-88CC

[English](README.md) | [中文](README.zh.md)

A high-performance Claude Code statusline tool written in Rust with Git integration, usage tracking, interactive TUI configuration, API quota monitoring and Claude Code enhancement utilities.

> **Maintained by [HoBeedzc](https://github.com/HoBeedzc)** - This is a specially adapted version of CCometixLine for 88Code service. The original CCometixLine was created by [Haleclipse](https://github.com/Haleclipse/CCometixLine) under MIT License. This project is also released under MIT License.
>
> This project also incorporates code from another MIT-licensed project, [ccline-packycc](https://github.com/ding113/ccline-packycc), with attribution retained.
>
> 88Code is a third-party Claude Code proxy service. This project is a voluntary third-party adaptation and is not affiliated with Anthropic or 88Code. 88Code websites: [88code](https://www.88code.org/). This project implements automatic adaptation for both endpoints.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
[![CI](https://github.com/byebye-code/ccline-88cc/actions/workflows/ci.yml/badge.svg)](https://github.com/byebye-code/ccline-88cc/actions/workflows/ci.yml)

## Screenshots

![CCometixLine](assets/img1.png)

The statusline shows: Model | Directory | Git Branch Status | Context Window | API Quota

## Features

### Core Functionality
- **Git integration** with branch, status, and tracking info  
- **Model display** with simplified Claude model names
- **Usage tracking** based on transcript analysis
- **Directory display** showing current workspace
- **API Quota display** showing current API quota
- **Minimal design** using Nerd Font icons

### Interactive TUI Features
- **Interactive main menu** when executed without input
- **TUI configuration interface** with real-time preview
- **Theme system** with multiple built-in presets
- **Segment customization** with granular control
- **Configuration management** (init, check, edit)

### Claude Code Enhancement
- **Context warning disabler** - Remove annoying "Context low" messages
- **Verbose mode enabler** - Enhanced output detail
- **Robust patcher** - Survives Claude Code version updates
- **Automatic backups** - Safe modification with easy recovery

## Installation

### Quick Install (Recommended)

Install via npm (works on all platforms):

```bash
# Install globally
npm install -g @byebyecode/ccline-88cc

# Or using yarn
yarn global add @byebyecode/ccline-88cc

# Or using pnpm
pnpm add -g @byebyecode/ccline-88cc
```

Use npm mirror for faster download:
```bash
npm install -g @byebyecode/ccline-88cc --registry https://registry.npmmirror.com
```

After installation:
- ✅ Global command `ccline-88cc` is available everywhere
- ⚙️ Follow the configuration steps below to integrate with Claude Code
- 🎨 Run `ccline-88cc -c` to open configuration panel for theme selection

### Claude Code Configuration

Add to your Claude Code `settings.json`:

**Linux/macOS:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "~/.claude/ccline/ccline-88cc",
    "padding": 0
  }
}
```

**Windows:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "%USERPROFILE%\\.claude\\ccline\\ccline-88cc.exe",
    "padding": 0
  }
}
```

**Fallback (npm installation):**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "ccline-88cc",
    "padding": 0
  }
}
```
*Use this if npm global installation is available in PATH*

### Update

```bash
npm update -g @byebyecode/ccline-88cc
```

<details>
<summary>Manual Installation (Click to expand)</summary>

Alternatively, download from [Releases](https://github.com/byebye-code/ccline-88cc/releases):

#### Linux

#### Option 1: Dynamic Binary (Recommended)
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/byebye-code/ccline-88cc/releases/latest/download/ccline-88cc-linux-x64.tar.gz
tar -xzf ccline-88cc-linux-x64.tar.gz
cp ccline-88cc ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline-88cc
```
*Requires: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### Option 2: Static Binary (Universal Compatibility)
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/byebye-code/ccline-88cc/releases/latest/download/ccline-88cc-linux-x64-static.tar.gz
tar -xzf ccline-88cc-linux-x64-static.tar.gz
cp ccline-88cc ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline-88cc
```
*Works on any Linux distribution (static, no dependencies)*

#### macOS (Intel)

```bash  
mkdir -p ~/.claude/ccline
wget https://github.com/byebye-code/ccline-88cc/releases/latest/download/ccline-88cc-macos-x64.tar.gz
tar -xzf ccline-88cc-macos-x64.tar.gz
cp ccline-88cc ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline-88cc
```

#### macOS (Apple Silicon)

```bash
mkdir -p ~/.claude/ccline  
wget https://github.com/byebye-code/ccline-88cc/releases/latest/download/ccline-88cc-macos-arm64.tar.gz
tar -xzf ccline-88cc-macos-arm64.tar.gz
cp ccline-88cc ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline-88cc
```

#### Windows

```powershell
# Create directory and download
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\ccline-88cc"
Invoke-WebRequest -Uri "https://github.com/byebye-code/ccline-88cc/releases/latest/download/ccline-88cc-windows-x64.zip" -OutFile "ccline-88cc-windows-x64.zip"
Expand-Archive -Path "ccline-88cc-windows-x64.zip" -DestinationPath "."
Move-Item "ccline-88cc.exe" "$env:USERPROFILE\.claude\ccline-88cc\"
```

</details>

### Build from Source

```bash
git clone https://github.com/byebye-code/ccline-88cc.git
cd ccline-88cc
cargo build --release

# Linux/macOS
mkdir -p ~/.claude/ccline
cp target/release/ccometixline ~/.claude/ccline-88cc/ccline-88cc
chmod +x ~/.claude/ccline/ccline-88cc

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\ccline-88cc"
copy target\release\ccometixline.exe "$env:USERPROFILE\.claude\ccline-88cc\ccline-88cc.exe"
```

## Usage

### Configuration Management

```bash
# Initialize configuration file
ccline-88cc --init

# Check configuration validity  
ccline-88cc --check

# Print current configuration
ccline-88cc --print

# Enter TUI configuration mode
ccline-88cc --config
```

### Theme Override

```bash
# Temporarily use specific theme (overrides config file)
ccline-88cc --theme cometix
ccline-88cc --theme minimal
ccline-88cc --theme gruvbox
ccline-88cc --theme nord
ccline-88cc --theme powerline-dark

# Or use custom theme files from ~/.claude/ccline-88cc/themes/
ccline-88cc --theme my-custom-theme
```

### Claude Code Enhancement

```bash
# Disable context warnings and enable verbose mode
ccline-88cc --patch /path/to/claude-code/cli.js

# Example for common installation
ccline-88cc --patch ~/.local/share/fnm/node-versions/v24.4.1/installation/lib/node_modules/@anthropic-ai/claude-code/cli.js
```

## Default Segments

Displays: `Directory | Git Branch Status | Model | Context Window | API Quota`

### Git Status Indicators

- Branch name with Nerd Font icon
- Status: `✓` Clean, `●` Dirty, `⚠` Conflicts  
- Remote tracking: `↑n` Ahead, `↓n` Behind

### Model Display

Shows simplified Claude model names:
- `claude-3-5-sonnet` → `Sonnet 3.5`
- `claude-4-sonnet` → `Sonnet 4`

### Context Window Display

Token usage percentage based on transcript analysis with context limit tracking.

### API Quota Display
Smart monitoring of API usage:

- **Usage display**: Shows subscription name and used/total credits (e.g., `Pro $0.06/$20.25`)
- **Auto-detection**: Automatically detects the correct API endpoint
- **Zero configuration**: Just provide your API key, everything else is automatic

Supports multiple API key sources:

- Environment variables: `C88_API_KEY`, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`
- Claude Code settings.json
- Local API key file: `~/.claude/api_key`

## Configuration

CCometixLine supports full configuration via TOML files and interactive TUI:

- **Configuration file**: `~/.claude/ccline-88cc/config.toml`
- **Interactive TUI**: `ccline-88cc --config` for real-time editing with preview
- **Theme files**: `~/.claude/ccline-88cc/themes/*.toml` for custom themes
- **Automatic initialization**: `ccline-88cc --init` creates default configuration

### Available Segments

All segments are configurable with:
- Enable/disable toggle
- Custom separators and icons
- Color customization
- Format options

Supported segments: Directory, Git, Model, Usage, Time, Cost, OutputStyle


## Requirements

- **Git**: Version 1.5+ (Git 2.22+ recommended for better branch detection)
- **Terminal**: Must support Nerd Fonts for proper icon display
  - Install a [Nerd Font](https://www.nerdfonts.com/) (e.g., FiraCode Nerd Font, JetBrains Mono Nerd Font)
  - Configure your terminal to use the Nerd Font
- **Claude Code**: For statusline integration

## Development

```bash
# Build development version
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## Roadmap

- [x] TOML configuration file support
- [x] TUI configuration interface
- [x] Custom themes
- [x] Interactive main menu
- [x] Claude Code enhancement tools

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Related Projects

- [tweakcc](https://github.com/Piebald-AI/tweakcc) - Command-line tool to customize your Claude Code themes, thinking verbs, and more.
- [CCometixLine](https://github.com/Haleclipse/CCometixLine) - Original high-performance Claude Code status line tool written in Rust (upstream project).
- [ccline-packycc](https://github.com/ding113/ccline-packycc) - Another high-performance Claude Code status line tool written in Rust.

## License

This project is licensed under the [MIT License](LICENSE).

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=byebye-code/ccline-88cc&type=Date)](https://star-history.com/#byebye-code/ccline-88cc&Date)
