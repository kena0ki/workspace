#!/bin/zsh

SAMPLE_DIR=./src/contest

. ./login.env

if [ `uname -m` = "arm64" ]; then # For M1 Mac
  export PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
  export PUPPETEER_EXECUTABLE_PATH=`which chromium`
fi

if [ -z "$1" ]; then
  echo "[ERROR] Input contest ID. (e.g. 1711)"
  exit
fi
node ./tool/get-samples.js "$1" "$SAMPLE_DIR"

