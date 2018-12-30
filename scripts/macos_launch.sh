#!/bin/bash
# This is used via build_macos.sh, not meant to be called directly

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BIN=energygrid-bin

$DIR/$BIN
