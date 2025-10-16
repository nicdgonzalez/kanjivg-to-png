#!/usr/bin/env bash

[ ! -z "${TRACE+x}" ] && set -x

main() {
    local -r output_dir="$1"

    local data="$(
        curl \
            --silent \
            --show-error \
            --location \
            -- \
            'https://api.github.com/repos/KanjiVG/kanjivg/releases/latest'
    )"
    local file_name="$(echo "$data" | jq '.assets[0].name' --raw-output)"
    local download_url="$(echo "$data" | jq '.assets[0].browser_download_url' --raw-output)"

    curl --silent --show-error --location --output "$file_name" -- "$download_url"
    mkdir --parents "$output_dir"
    unzip "$file_name" -d "$output_dir"
    rm "$file_name"

    # This tells the other scripts where to look for the KanjiVG SVGs.
    export KVG_LOOKUP="$output/kanji"
}

main "$@"
