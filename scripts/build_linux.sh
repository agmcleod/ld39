#!/bin/bash

rm *.AppImage
rm -rf EnergyGrid.AppDir
cargo build --release
mkdir -p EnergyGrid.AppDir
cp -r resources EnergyGrid.AppDir

cp target/release/ld39 EnergyGrid.AppDir/AppRun

cd EnergyGrid.AppDir
mv resources/logo.png energygrid.png

echo '[Desktop Entry]' > energygrid.desktop
echo 'Name=EnergyGrid' >> energygrid.desktop
echo 'Exec=EnergyGrid' >> energygrid.desktop
echo 'Icon=energygrid' >> energygrid.desktop
echo 'Type=Application' >> energygrid.desktop
echo 'Categories=Game;' >> energygrid.desktop

cd ..
appimagetool-x86_64.AppImage EnergyGrid.AppDir
