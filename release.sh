#!/bin/bash

# Release script for Windows Music Hotkey Mapper
# Creates a new version tag and triggers GitHub Actions release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo -e "${GREEN}Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Show options
echo ""
echo "Select version bump type:"
echo "  1) patch  ($MAJOR.$MINOR.$((PATCH + 1)))"
echo "  2) minor  ($MAJOR.$((MINOR + 1)).0)"
echo "  3) major  ($((MAJOR + 1)).0.0)"
echo "  4) custom (enter manually)"
echo ""

read -p "Choice [1-4]: " CHOICE

case $CHOICE in
    1)
        NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
        ;;
    2)
        NEW_VERSION="$MAJOR.$((MINOR + 1)).0"
        ;;
    3)
        NEW_VERSION="$((MAJOR + 1)).0.0"
        ;;
    4)
        read -p "Enter new version (without 'v' prefix): " NEW_VERSION
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

TAG="v$NEW_VERSION"

echo ""
echo -e "${GREEN}New version: ${YELLOW}$NEW_VERSION${NC}"
echo -e "${GREEN}New tag: ${YELLOW}$TAG${NC}"
echo ""

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag $TAG already exists!${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}Warning: You have uncommitted changes.${NC}"
    read -p "Continue anyway? [y/N]: " CONTINUE
    if [ "$CONTINUE" != "y" ] && [ "$CONTINUE" != "Y" ]; then
        echo "Aborted."
        exit 1
    fi
fi

read -p "Proceed with release? [y/N]: " CONFIRM
if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    echo "Aborted."
    exit 0
fi

# Update version in Cargo.toml
echo -e "${GREEN}Updating Cargo.toml...${NC}"
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Commit the version bump
echo -e "${GREEN}Creating commit...${NC}"
git add Cargo.toml
git commit -m "chore: bump version to $NEW_VERSION"

# Create tag
echo -e "${GREEN}Creating tag $TAG...${NC}"
git tag -a "$TAG" -m "Release $NEW_VERSION"

# Push changes and tag
echo -e "${GREEN}Pushing to remote...${NC}"
git push origin HEAD
git push origin "$TAG"

echo ""
echo -e "${GREEN}Done! Release $TAG has been created and pushed.${NC}"
echo -e "${GREEN}GitHub Actions will now build and create the release.${NC}"
echo ""
echo -e "Check the progress at: ${YELLOW}https://github.com/alexeyleping/WindowsMusicHotkeyMapper/actions${NC}"
