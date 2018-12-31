#!/bin/bash

rm -rf winbuild/
./scripts/copy_resources.sh
cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
mkdir -p winbuild
cp -r resources winbuild/
cp target/release/ld39.exe winbuild/EnerygGrid.exe
mv winbuild/resources/logo.ico winbuild/logo.ico

cd winbuild
# Expects https://github.com/electron/rcedit to be on Path
rcedit "EnerygGrid.exe" --set-icon "logo.ico"
rm logo.ico