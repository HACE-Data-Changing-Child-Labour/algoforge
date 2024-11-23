#!/usr/bin/env bash
set -e

usage() {
    echo "Usage: $0 [major|minor|patch]"
    exit 1
}

if [ $# -ne 1 ]; then
    usage
fi

BUMP_TYPE=$1

if [[ "$BUMP_TYPE" != "major" && "$BUMP_TYPE" != "minor" && "$BUMP_TYPE" != "patch" ]]; then
    usage
fi

PYPROJECT_FILE="pyproject.toml"
CARGO_TOML_FILE="Cargo.toml"

# Extract the current version from the file
CURRENT_VERSION=$(grep -E '^version = ' "$CARGO_TOML_FILE" | sed -E 's/version = "(.*)"/\1/')

if [ "$CURRENT_VERSION" = "" ]; then
    echo "Error: Could not find the version in $CARGO_TOML_FILE"
    exit 1
fi

echo "Current version: $CURRENT_VERSION"

# Split the version into an array
IFS='.' read -ra VERSION_PARTS <<<"$CURRENT_VERSION"

# Ensure we have exactly three components
if [ ${#VERSION_PARTS[@]} -ne 3 ]; then
    echo "Error: Version number is not in MAJOR.MINOR.PATCH format"
    exit 1
fi

MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

# Increment the version based on the input
case $BUMP_TYPE in
major)
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
    ;;
minor)
    MINOR=$((MINOR + 1))
    PATCH=0
    ;;
patch)
    PATCH=$((PATCH + 1))
    ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo "New version: $NEW_VERSION"

# Update the version in the file
sed -i.bak -E "s/(^version = ).*/\1\"$NEW_VERSION\"/" "$CARGO_TOML_FILE"
rm "${CARGO_TOML_FILE}.bak"

sed -i.bak -E "s/(^version = ).*/\1\"$NEW_VERSION\"/" "$PYPROJECT_FILE"
rm "${PYPROJECT_FILE}.bak"

# Commit the changes
git add "$CARGO_TOML_FILE" "$PYPROJECT_FILE"
git commit -m "Bump version to v$NEW_VERSION"

# Create a new tag
git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

# Push changes to the remote repository
git push origin main
git push origin "v$NEW_VERSION"

echo "Successfully released version $NEW_VERSION"
