#!/usr/bin/env bash
cd www/ && npm run build && rm -rf ../docs && cp -r dist ../docs && cp -r roms ../docs