#!/usr/bin/bash
cd levenshtein-ext && cargo build && cd ..
php -d extension=./levenshtein-ext/target/debug/liblevenshtein_ext.so test.php
