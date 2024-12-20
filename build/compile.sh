#!/bin/bash
set -euo pipefail

# Configuration
OUTPUT_DIR="output"
DEFAULT_ARCH="amd64"

# Target architecture mapping
declare -A TARGETS=(
    ["amd64"]="x86_64-unknown-linux-gnu"
    ["arm64"]="aarch64-unknown-linux-gnu"
)

# Help message
show_usage() {
    echo "Usage: $0 [ARCH]"
    echo "Supported architectures:"
    for arch in "${!TARGETS[@]}"; do
        echo "  - $arch"
    done
    exit 1
}

# Get all binary names from Cargo.toml
get_binary_names() {
    # Parse binary names from Cargo.toml
    cargo metadata --no-deps --format-version 1 | \
        jq -r '.packages[0].targets[] | select(.kind[] | contains("bin")) | .name'
}

# Compile for specific target
compile() {
    local target="$1"
    echo "Compiling for target: $target"

    # Add target and build
    rustup target add "$target"
    RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --all --target="$target"

    # Prepare output directory
    rm -rf "$OUTPUT_DIR"
    mkdir -p "$OUTPUT_DIR"

    # Get all binary names
    local binaries
    readarray -t binaries < <(get_binary_names)
    
    # Copy and compress each binary
    for binary in "${binaries[@]}"; do
        echo "Processing binary: $binary"
        
        # Check if binary exists
        local binary_path="target/$target/release/$binary"
        if [[ ! -f "$binary_path" ]]; then
            echo "Warning: Binary not found: $binary_path"
            continue
        fi

        # Copy binary
        cp "$binary_path" "$OUTPUT_DIR/$binary"
        
        # Compress with UPX
        echo "Compressing $binary with UPX..."
        upx --lzma --best -q "$OUTPUT_DIR/$binary"
        upx -t "$OUTPUT_DIR/$binary"
        
        echo "Successfully processed: $binary"
    done

    echo "Build complete for $target. Binaries available in $OUTPUT_DIR/"
}

main() {
    # Get architecture argument or use default
    ARCH="${1:-$DEFAULT_ARCH}"

    # Validate architecture
    if [[ ! ${TARGETS[$ARCH]+_} ]]; then
        echo "Error: Unsupported architecture: $ARCH"
        show_usage
    fi

    # Compile for target architecture
    compile "${TARGETS[$ARCH]}"
}

main "$@"
