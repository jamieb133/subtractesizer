#!/bin/bash

cp target/debug/libsubtractesizer.so gui/subtractesizer.so
python3 gui/frontend.py
