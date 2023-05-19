#!/bin/sh

set -xe -o pipefail

TARGET_JSON_NAME="cppreference.json"
TARGET_REFERENCE_DIR="cppreference"
DATASET_GENERATOR_DIR="./dataset-generator"
DATASET_GENERATOR="${DATASET_GENERATOR_DIR}/target/release/dataset-generator"

if [ -e "${TARGET_REFERENCE_DIR}" ]; then
	echo "${TARGET_REFERENCE_DIR} directory already exist, skipping"
else
	wget https://github.com/PeterFeicht/cppreference-doc/releases/download/v20220730/html-book-20220730.zip -O cppreference.zip
	unzip cppreference.zip 'reference/*'
	mv reference "${TARGET_REFERENCE_DIR}"
fi

if [ ! -e "${DATASET_GENERATOR}" ]; then
	cd "${DATASET_GENERATOR_DIR}"
	cargo build --release
fi

"${DATASET_GENERATOR}" --cppreference "${TARGET_REFERENCE_DIR}" -o cppreference.json

