#!/usr/bin/env bash
targets=(
  darwin-x64
  darwin-arm64
  win32-x64
  win32-arm64
  win32-ia32
  linux-x64
  linux-arm64
  linux-arm
  android-x4
  android-arm64
  android-arm
)

BASE_DIR=packages/@naql

for target in "${targets[@]}"; do
  mkdir -p $BASE_DIR/"cli-$target"
  cd $BASE_DIR/cli-"$target" || exit
  yarn init --name @naql/cli-"$target"
  cd - || exit
done
