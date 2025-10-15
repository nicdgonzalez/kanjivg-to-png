#!/usr/bin/env bash

[ ! -z "${TRACE+x}" ] && set -x

main() {
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
    unzip "$file_name"
    rm "$file_name"
}

main "$@"
