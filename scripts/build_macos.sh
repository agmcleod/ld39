#!/bin/bash

set -e

APP_NAME=ld39
MACOS_BIN_NAME=energygrid-bin
MACOS_APP_NAME=EnergyGrid
MACOS_APP_DIR=$MACOS_APP_NAME.app
RESOURCES=resources

echo "Creating app directory structure"
rm -rf $MACOS_APP_NAME
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo rustc \
    --verbose \
    --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying resources directory"
cp -r $RESOURCES $MACOS_APP_DIR/Contents/MacOS

echo "Copying launcher"
cp scripts/macos_launch.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Creating dmg"
mkdir -p $MACOS_APP_NAME
cp -r $MACOS_APP_DIR $MACOS_APP_NAME/
ln -s /Applications $MACOS_APP_NAME/Applications
rm -rf $MACOS_APP_NAME/.Trashes

FULL_NAME=$MACOS_APP_NAME

mkdir -p uploads
hdiutil create uploads/$FULL_NAME.dmg -srcfolder $MACOS_APP_NAME -ov
rm -rf $MACOS_APP_NAME
