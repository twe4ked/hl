#!/bin/sh

cargo run -- \
  --red foo\ bar\ baz\
  --blue ba \
  --green bar \
  < fixtures/example.txt \
  2> /dev/null
