# Zed Function Extractor Extension

This extension for the Zed editor extracts function declarations from various programming language files and displays them in a convenient list. It uses language servers for accurate code analysis rather than regex-based parsing.

## Features

- Extracts function declarations from:
  - Python (using pyright-langserver)
  - JavaScript/TypeScript (using typescript-language-server)
  - C/C++ (using clangd)
  - Rust (using rust-analyzer)
- Shows line numbers for each function
- Displays the full function signature
- Organizes functions by file

## Building from Source

### Prerequisites

1. **Install Rust and Cargo**
   ```bash
   # Install rustup (Rust version manager)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Follow the prompts to complete installation
   # Choose option 1 for default installation
   
   # Add Rust to your current shell session
   source "$HOME/.cargo/env"
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Install Build Dependencies**
   ```bash
   # For Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install -y build-essential pkg-config
   
   # For macOS
   xcode-select --install
   
   # For Windows
   # Install Visual Studio Build Tools with C++ support
   ```

3. **Install Zed Editor**
   - Download from [zed.dev](https://zed.dev)
   - Follow installation instructions for your platform

### Building the Extension

1. **Clone or Download the Repository**
   ```bash
   git clone https://github.com/your-username/zed-function-extractor.git
   cd zed-function-extractor
   ```

2. **Update Dependencies (if needed)**
   ```bash
   # Check Cargo.toml to ensure it contains:
   # zed_extension_api = "0.3.0"
   # serde_json = "1.0"
   
   # Update dependencies if needed
   cargo update
   ```

3. **Build the Extension**
   ```bash
   # Development build
   cargo build
   
   # Release build (recommended for installation)
   cargo build --release
   ```

4. **Prepare Extension for Installation**
   ```bash
   # Create distribution directory
   mkdir -p dist
   
   # Copy necessary files
   cp extension.toml dist/
   cp README.md dist/
   cp target/release/libfunction_extractor.so dist/
   ```

### Installing in Zed

1. Open Zed editor
2. Open the Command Palette (Cmd+Shift+P or Ctrl+Shift+P)
3. Type "Extensions: Install Extension Development Folder"
4. Select the `dist` directory you created

## Usage

1. Open a supported file (Python, JavaScript/TypeScript, C/C++, or Rust)
2. Run the command "Extract Function Declarations" from the command palette
3. A new buffer will open showing all detected functions with their line numbers and signatures

## Supported Languages and Language Servers

### Python
- Uses pyright-langserver
- Detects functions, methods, and class methods

### JavaScript/TypeScript
- Uses typescript-language-server
- Detects regular functions, arrow functions, class methods, and async functions

### C/C++
- Uses clangd
- Detects function definitions, class methods, constructors/destructors

### Rust
- Uses rust-analyzer
- Detects functions, methods, associated functions, and closures

## Troubleshooting

### Language Server Installation

For the extension to work properly, you need to have the appropriate language servers installed:

1. **Python (pyright)**
   ```bash
   npm install -g pyright
   ```

2. **TypeScript/JavaScript**
   ```bash
   npm install -g typescript typescript-language-server
   ```

3. **C/C++ (clangd)**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install clangd
   
   # macOS
   brew install llvm
   
   # Windows
   # Install LLVM from https://releases.llvm.org/
   ```

4. **Rust (rust-analyzer)**
   ```bash
   # Using rustup
   rustup component add rust-analyzer
   
   # Or download from https://github.com/rust-analyzer/rust-analyzer/releases
   ```

### Common Issues

- **Extension not appearing in Zed**: Make sure the `dist` directory contains all required files.
- **Functions not being detected**: Verify that the appropriate language server is installed and in your PATH.
- **Build errors**: Check that you have the correct version of Rust and all dependencies installed.

## Development

This extension is built using:
- Rust
- Zed Extension API (v0.3.0)
- Language Server Protocol (LSP)

To modify the extension:
1. Edit the source code in `src/lib.rs`
2. Rebuild using `cargo build`
3. Copy the new binary to the `dist` directory
4. Reinstall the extension in Zed

## License

MIT

