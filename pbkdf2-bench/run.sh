#!/bin/sh
for x in 1 2 3 4 5 ; do cargo run --release > out.$x ; done
