#!/bin/bash

cd $PROJECT_ROOT
cargo build
cp target/debug/libsubtractesizer.so gui/subtractesizer.so
python3 gui/frontend.py
