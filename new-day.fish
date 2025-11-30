#!/usr/bin/env fish

set day (printf "%02d" $argv[1])

cp ./sample/day00.rs src/day$day.rs
nvim resources/day$day-input.txt
