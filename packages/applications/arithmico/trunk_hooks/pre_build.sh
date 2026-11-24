#!/bin/bash

find ../../.. -type f -name "*.css" \
    ! -path "*/dist/*" ! -name "index.css" \
    ! -path "*/node_modules/*" \
    -exec cat {} + > index.css
