#!/usr/bin/bash
cargo build
php -d extension=./levenshtein-ext/target/debug/liblevenshtein_ext.so test.php
