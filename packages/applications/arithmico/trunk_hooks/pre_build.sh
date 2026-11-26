#!/bin/bash

find ../../.. -type f -name "*.css" \
    ! -path "*/dist/*" \
    ! -path "*/node_modules/*" \
    ! -path "*/infrastructure/*" \
    ! -path "*/target/*" \
    ! -name "index.css" \
    -exec cat {} + > index.css
