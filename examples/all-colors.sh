#!/bin/sh

cargo run -- \
  --blue a\
  --black b \
  --cyan c \
  --dark-blue d \
  --dark-cyan e \
  --dark-green f \
  --dark-grey g \
  --dark-magenta h \
  --dark-red i \
  --dark-yellow j \
  --green k \
  --grey l \
  --magenta m \
  --red n \
  --white o \
  --yellow p \
  < fixtures/alphabet.txt \
  2> /dev/null
