#!/bin/bash
set -e

echo "Building Janus for all platforms..."

# Build for macOS ARM64
echo "Building for macOS ARM64..."
cargo build --release --target aarch64-apple-darwin
mkdir -p npm/platforms/darwin-arm64/bin
cp target/aarch64-apple-darwin/release/harness-cli npm/platforms/darwin-arm64/bin/

# Build for macOS x64
echo "Building for macOS x64..."
cargo build --release --target x86_64-apple-darwin
mkdir -p npm/platforms/darwin-x64/bin
cp target/x86_64-apple-darwin/release/harness-cli npm/platforms/darwin-x64/bin/

# Build for Linux x64
echo "Building for Linux x64..."
cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p npm/platforms/linux-x64/bin
cp target/x86_64-unknown-linux-gnu/release/harness-cli npm/platforms/linux-x64/bin/

# Build for Windows x64
echo "Building for Windows x64..."
cargo build --release --target x86_64-pc-windows-msvc
mkdir -p npm/platforms/win32-x64/bin
cp target/x86_64-pc-windows-msvc/release/harness-cli.exe npm/platforms/win32-x64/bin/

echo "All builds complete!"
echo ""
echo "To publish:"
echo "  cd npm/platforms/darwin-arm64 && npm publish --access public"
echo "  cd npm/platforms/darwin-x64 && npm publish --access public"
echo "  cd npm/platforms/linux-x64 && npm publish --access public"
echo "  cd npm/platforms/win32-x64 && npm publish --access public"
echo "  cd npm && npm publish --access public"
