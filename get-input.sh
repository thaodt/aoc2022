#!/bin/bash

year=2022

day=$(date +%-d)
dir=day$day

cargo new $dir
curl -b "$(cat ~/data/aoc_hd)" 'https://adventofcode.com/$year/day/$day/input' \
     > $dir/input

cp template.rs $dir/src/main.rs
