#!/bin/bash
source /usr/share/nvm/init-nvm.sh
systemfd --no-pid -s https::0.0.0.0:8000 -- cargo watch  --use-shell=/bin/bash -c --quiet --no-gitignore --no-ignore -d 0 -w '.trigger' -s 'cargo run --package backend --bin stellarust --release';