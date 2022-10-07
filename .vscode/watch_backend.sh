#!/bin/bash
source /usr/share/nvm/init-nvm.sh;
cargo watch --use-shell=/bin/bash -c --quiet -d 0 --no-gitignore -s  ' cargo check --all' -s 'touch .trigger'