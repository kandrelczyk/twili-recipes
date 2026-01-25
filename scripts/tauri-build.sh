#!/bin/bash
# GitLab CI/CD equivalent of tauri-apps/tauri-action
# Builds Tauri app, collects artifacts, generates latest.json, and uploads to GitLab release

set -e

# =============================================================================
# Configuration
# =============================================================================
PROJECT_PATH="${PROJECT_PATH:-.}"
TAURI_SCRIPT="${TAURI_SCRIPT:-cargo tauri}"
BUILD_ARGS="${BUILD_ARGS:-}"
INCLUDE_UPDATER_JSON="${INCLUDE_UPDATER_JSON:-true}"
PLATFORM="${PLATFORM:-linux}"  # linux or android

# GitLab variables (should be set by CI)
# CI_API_V4_URL, CI_PROJECT_ID, GITLAB_TOKEN, APP_VERSION

# =============================================================================
# Helper Functions
# =============================================================================

log() {
    echo "[tauri-build] $1"
}

error() {
    echo "[tauri-build] ERROR: $1" >&2
    exit 1
}

get_version() {
    node -p "require('./src-tauri/tauri.conf.json').version"
}

get_app_name() {
    node -p "require('./src-tauri/tauri.conf.json').productName || require('./src-tauri/tauri.conf.json').package?.productName || 'app'"
}

# Normalize architecture names
normalize_arch() {
    local arch="$1"
    case "$arch" in
        amd64|x86_64|x64) echo "x86_64" ;;
        i386|i686|x86|x32) echo "i686" ;;
        arm64|aarch64) echo "aarch64" ;;
        arm|armv7|armhf) echo "armv7" ;;
        *) echo "$arch" ;;
    esac
}

# Get Debian architecture naming
get_deb_arch() {
    local arch="$1"
    case "$arch" in
        x86_64|amd64|x64) echo "amd64" ;;
        i686|i386|x86|x32) echo "i386" ;;
        aarch64|arm64) echo "arm64" ;;
        armv7|arm|armhf) echo "armhf" ;;
        *) echo "$arch" ;;
    esac
}

# Get AppImage architecture naming
get_appimage_arch() {
    local arch="$1"
    case "$arch" in
        x86_64|amd64|x64) echo "amd64" ;;
        i686|i386|x86|x32) echo "i386" ;;
        aarch64|arm64) echo "aarch64" ;;
        armv7|arm|armhf) echo "arm" ;;
        *) echo "$arch" ;;
    esac
}

# =============================================================================
# Build Functions
# =============================================================================

build_tauri() {
    log "Building Tauri application..."

    cd "$PROJECT_PATH"

    # Run the build
    $TAURI_SCRIPT build $BUILD_ARGS

    log "Build completed successfully"
}

# =============================================================================
# Artifact Collection
# =============================================================================

collect_linux_artifacts() {
    local version="$1"
    local app_name="$2"
    local artifacts_dir="$3"

    local arch=$(uname -m)
    local deb_arch=$(get_deb_arch "$arch")
    local appimage_arch=$(get_appimage_arch "$arch")
    local normalized_arch=$(normalize_arch "$arch")

    # Convert app name to kebab-case for Linux file names
    local linux_app_name=$(echo "$app_name" | sed -E 's/([a-z0-9])([A-Z])/\1-\2/g; s/([A-Z])([A-Z])([a-z])/\1-\2\3/g' | tr '[:upper:]' '[:lower:]' | tr ' _.' '-' | sed 's/[()[\]{}]//g')

    local bundle_dir="target/release/bundle"

    log "Collecting Linux artifacts from $bundle_dir..."
    log "Looking for app: $app_name (linux name: $linux_app_name), version: $version"

    mkdir -p "$artifacts_dir"

    # Collect .deb files
    for pattern in "$app_name" "$linux_app_name"; do
        for file in "$bundle_dir/deb/${pattern}_${version}_${deb_arch}.deb" \
                    "$bundle_dir/deb/${pattern}_${version}_${deb_arch}.deb.sig"; do
            if [ -f "$file" ]; then
                cp "$file" "$artifacts_dir/"
                log "Found: $(basename "$file")"
            fi
        done
    done

    # Collect .AppImage files
    for pattern in "$app_name" "$linux_app_name"; do
        for file in "$bundle_dir/appimage/${pattern}_${version}_${appimage_arch}.AppImage" \
                    "$bundle_dir/appimage/${pattern}_${version}_${appimage_arch}.AppImage.sig" \
                    "$bundle_dir/appimage/${pattern}_${version}_${appimage_arch}.AppImage.tar.gz" \
                    "$bundle_dir/appimage/${pattern}_${version}_${appimage_arch}.AppImage.tar.gz.sig"; do
            if [ -f "$file" ]; then
                cp "$file" "$artifacts_dir/"
                log "Found: $(basename "$file")"
            fi
        done
    done

    # Collect .rpm files (if exists)
    for pattern in "$app_name" "$linux_app_name"; do
        for file in "$bundle_dir/rpm/${pattern}-${version}"*.rpm \
                    "$bundle_dir/rpm/${pattern}-${version}"*.rpm.sig; do
            if [ -f "$file" ]; then
                cp "$file" "$artifacts_dir/"
                log "Found: $(basename "$file")"
            fi
        done
    done

    # List all collected artifacts
    log "Collected artifacts:"
    ls -la "$artifacts_dir/" 2>/dev/null || log "No artifacts found"
}

collect_android_artifacts() {
    local version="$1"
    local app_name="$2"
    local artifacts_dir="$3"

    log "Collecting Android artifacts..."

    mkdir -p "$artifacts_dir"

    # Find APK files
    local apk_dir="src-tauri/gen/android/app/build/outputs/apk"

    for file in $(find "$apk_dir" -name "*.apk" 2>/dev/null); do
        cp "$file" "$artifacts_dir/"
        log "Found: $(basename "$file")"
    done

    # List all collected artifacts
    log "Collected artifacts:"
    ls -la "$artifacts_dir/" 2>/dev/null || log "No artifacts found"
}

# =============================================================================
# latest.json Generation
# =============================================================================

generate_latest_json() {
    local version="$1"
    local notes="$2"
    local artifacts_dir="$3"
    local base_url="$4"
    local output_file="$5"

    log "Generating latest.json..."

    local pub_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local arch=$(uname -m)
    local normalized_arch=$(normalize_arch "$arch")

    # Start building JSON
    local platforms_json=""
    local first_entry=true

    # Helper to add platform entry
    add_platform() {
        local platform_name="$1"
        local file="$2"
        local sig_file="${file}.sig"

        if [ -f "$file" ] && [ -f "$sig_file" ]; then
            local filename=$(basename "$file")
            local signature=$(cat "$sig_file")
            local download_url="${base_url}/${filename}"

            if [ "$first_entry" = false ]; then
                platforms_json="${platforms_json},"
            fi
            first_entry=false

            platforms_json="${platforms_json}
    \"${platform_name}\": {
      \"signature\": \"${signature}\",
      \"url\": \"${download_url}\"
    }"
            log "Added platform: $platform_name -> $filename"
        fi
    }

    # Find and add AppImage
    for file in "$artifacts_dir"/*.AppImage; do
        if [ -f "$file" ]; then
            add_platform "linux-${normalized_arch}-appimage" "$file"
            break
        fi
    done

    # Find and add .deb
    for file in "$artifacts_dir"/*.deb; do
        if [ -f "$file" ]; then
            add_platform "linux-${normalized_arch}-deb" "$file"
            break
        fi
    done

    # Find and add .rpm
    for file in "$artifacts_dir"/*.rpm; do
        if [ -f "$file" ]; then
            add_platform "linux-${normalized_arch}-rpm" "$file"
            break
        fi
    done

    # Escape notes for JSON
    local escaped_notes=$(echo "$notes" | jq -Rs . | sed 's/^"//;s/"$//')

    # Generate final JSON
    cat > "$output_file" <<EOF
{
  "version": "$version",
  "notes": "$escaped_notes",
  "pub_date": "$pub_date",
  "platforms": {
$platforms_json
  }
}
EOF

    log "Generated latest.json:"
    cat "$output_file"
}

# =============================================================================
# GitLab Upload Functions
# =============================================================================

upload_to_gitlab_release() {
    local version="$1"
    local artifacts_dir="$2"

    if [ -z "$GITLAB_TOKEN" ] || [ -z "$CI_API_V4_URL" ] || [ -z "$CI_PROJECT_ID" ]; then
        log "GitLab variables not set, skipping upload"
        return 0
    fi

    log "Uploading artifacts to GitLab release $version..."

    # Upload each file
    for file in "$artifacts_dir"/*; do
        if [ -f "$file" ]; then
            local filename=$(basename "$file")
            log "Uploading $filename..."

            # Upload file to project uploads
            local upload_response=$(curl --silent --request POST \
                --header "PRIVATE-TOKEN: ${GITLAB_TOKEN}" \
                --form "file=@${file}" \
                "${CI_API_V4_URL}/projects/${CI_PROJECT_ID}/uploads")

            local file_url=$(echo "$upload_response" | jq -r '.full_path // empty')

            if [ -n "$file_url" ]; then
                log "Uploaded: $filename -> $file_url"

                # Add as release link
                curl --silent --request POST \
                    --header "PRIVATE-TOKEN: ${GITLAB_TOKEN}" \
                    --header "Content-Type: application/json" \
                    --data "{
                        \"name\": \"$filename\",
                        \"url\": \"${CI_SERVER_URL}${file_url}\",
                        \"link_type\": \"other\"
                    }" \
                    "${CI_API_V4_URL}/projects/${CI_PROJECT_ID}/releases/${version}/assets/links" || true
            else
                log "Warning: Failed to upload $filename"
            fi
        fi
    done

    log "Upload completed"
}

# =============================================================================
# Main
# =============================================================================

main() {
    local action="${1:-build}"

    cd "$PROJECT_PATH"

    local version=$(get_version)
    local app_name=$(get_app_name)
    local artifacts_dir="${ARTIFACTS_DIR:-./build-artifacts}"

    log "App: $app_name v$version"
    log "Platform: $PLATFORM"
    log "Action: $action"

    case "$action" in
        build)
            build_tauri
            ;;
        collect)
            if [ "$PLATFORM" = "android" ]; then
                collect_android_artifacts "$version" "$app_name" "$artifacts_dir"
            else
                collect_linux_artifacts "$version" "$app_name" "$artifacts_dir"
            fi
            ;;
        generate-json)
            local notes="${RELEASE_NOTES:-}"
            if [ -f "latest.md" ]; then
                notes=$(cat latest.md)
            fi
            local base_url="${DOWNLOAD_BASE_URL:-${CI_SERVER_URL}/${CI_PROJECT_PATH}/-/releases/${version}/downloads}"
            generate_latest_json "$version" "$notes" "$artifacts_dir" "$base_url" "$artifacts_dir/latest.json"
            ;;
        upload)
            upload_to_gitlab_release "$version" "$artifacts_dir"
            ;;
        all)
            build_tauri
            if [ "$PLATFORM" = "android" ]; then
                collect_android_artifacts "$version" "$app_name" "$artifacts_dir"
            else
                collect_linux_artifacts "$version" "$app_name" "$artifacts_dir"
                if [ "$INCLUDE_UPDATER_JSON" = "true" ]; then
                    local notes=""
                    if [ -f "latest.md" ]; then
                        notes=$(cat latest.md)
                    fi
                    local base_url="${DOWNLOAD_BASE_URL:-${CI_SERVER_URL}/${CI_PROJECT_PATH}/-/releases/${version}/downloads}"
                    generate_latest_json "$version" "$notes" "$artifacts_dir" "$base_url" "$artifacts_dir/latest.json"
                fi
            fi
            upload_to_gitlab_release "$version" "$artifacts_dir"
            ;;
        *)
            error "Unknown action: $action. Use: build, collect, generate-json, upload, or all"
            ;;
    esac
}

# Run main function with all arguments
main "$@"
