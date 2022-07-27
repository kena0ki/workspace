#!/bin/zsh

SAMPLE_DIR=./src/contest

. ./login.env
if [ -z "$1" ]; then
  echo "[ERROR] Input contest ID"
  exit
fi
node ./tool/get-samples.js "$1" "$SAMPLE_DIR"

