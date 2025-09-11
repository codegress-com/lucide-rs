#!/bin/bash

# Generate Lucide Icons Script
# This script:
# 1) Downloads all SVG icons from the Lucide Icons GitHub repository
# 2) Generates Rust components for all frameworks
# 3) Updates module exports
# 4) Cleans up temporary files

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$PROJECT_ROOT/packages/lucide"
ICONS_DIR="$PACKAGE_DIR/icons"
TEMP_DIR="$(mktemp -d)"

echo -e "${GREEN}🚀 Starting Lucide Icons generation...${NC}"

# Function to cleanup temp directory on exit
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
    rm -rf "$TEMP_DIR"
    # Clean up icons directory after generation
    if [ -d "$ICONS_DIR" ]; then
        echo -e "${YELLOW}🗑️  Removing icons directory...${NC}"
        rm -rf "$ICONS_DIR"
    fi
}
trap cleanup EXIT

echo -e "${BLUE}Step 1: Creating icons directory${NC}"
# Create icons directory if it doesn't exist
mkdir -p "$ICONS_DIR"

echo -e "${BLUE}Step 2: Downloading icons${NC}"
echo -e "${GREEN}📥 Downloading Lucide repository...${NC}"

# Clone the Lucide repository (sparse checkout for icons only)
cd "$TEMP_DIR"
git clone --depth 1 --filter=blob:none --sparse https://github.com/lucide-icons/lucide.git
cd lucide
git sparse-checkout set icons

# Check if icons directory exists in the repo
if [ ! -d "icons" ]; then
    echo -e "${RED}❌ Error: Icons directory not found in repository${NC}"
    exit 1
fi

# Count total icons
TOTAL_ICONS=$(find icons -name "*.svg" | wc -l | tr -d ' ')
echo -e "${GREEN}📊 Found $TOTAL_ICONS icons to download${NC}"

# Copy all SVG and JSON files from the repository to our icons directory
echo -e "${GREEN}📋 Copying icons to $ICONS_DIR...${NC}"
cp icons/*.svg "$ICONS_DIR/"
cp icons/*.json "$ICONS_DIR/" 2>/dev/null || echo -e "${YELLOW}⚠️  No JSON metadata files found${NC}"

# Count downloaded icons
DOWNLOADED_ICONS=$(find "$ICONS_DIR" -name "*.svg" | wc -l | tr -d ' ')

echo -e "${GREEN}✅ Successfully downloaded $DOWNLOADED_ICONS icons!${NC}"

echo -e "${BLUE}Step 3: Generating Rust components${NC}"
# Change to the package directory and run the code generator
cd "$PACKAGE_DIR"

# Build and run the code generator
echo -e "${GREEN}🔧 Building code generator...${NC}"
cargo build --bin generate-icons --features codegen --release

echo -e "${GREEN}⚙️  Running code generator...${NC}"
cargo run --bin generate-icons --features codegen --release

echo -e "${BLUE}Step 4: Cleanup will happen automatically${NC}"
echo -e "${GREEN}🎉 Generation complete!${NC}"
echo -e "${GREEN}📈 Generated components for $DOWNLOADED_ICONS icons across 4 frameworks${NC}"
