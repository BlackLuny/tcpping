#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print usage
usage() {
    echo -e "Usage: $0 [options] <version>"
    echo -e "Options:"
    echo -e "  -h, --help     Show this help message"
    echo -e "  -p, --prefix   Version prefix (default: 'v')"
    echo -e "\nExample:"
    echo -e "  $0 1.2.3       # Creates or updates v1.2.3"
    echo -e "  $0 -p r 2.0.0  # Creates or updates r2.0.0"
    exit 1
}

# Parse arguments
PREFIX="v"

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            ;;
        -p|--prefix)
            PREFIX="$2"
            shift 2
            ;;
        *)
            VERSION="$1"
            shift
            ;;
    esac
done

# Check if version is provided
if [ -z "$VERSION" ]; then
    echo -e "${RED}Error: Version number is required${NC}"
    usage
fi

# Validate version format (x.y.z)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Version must be in format x.y.z (e.g., 1.2.3)${NC}"
    exit 1
fi

# Construct tag
TAG="${PREFIX}${VERSION}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Check if working directory is clean
if ! git diff --quiet HEAD; then
    echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
    read -p "Do you want to continue? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag exists and delete it
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${YELLOW}Tag $TAG already exists, updating...${NC}"
    git tag -d "$TAG"
    git push origin ":$TAG" 2>/dev/null || true
fi

# Create and push tag
echo -e "${GREEN}Creating tag $TAG...${NC}"
git tag "$TAG"

echo -e "${GREEN}Pushing tag to origin...${NC}"
git push origin "$TAG"

echo -e "${GREEN}Release process started!${NC}"
echo -e "Check the GitHub Actions workflow at:"
echo -e "https://github.com/$(git config --get remote.origin.url | sed 's/.*://;s/.git$//')/actions"
