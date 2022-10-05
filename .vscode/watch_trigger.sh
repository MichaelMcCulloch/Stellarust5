#!/bin/bash

systemfd --no-pid -s https::0.0.0.0:8000 -- cargo watch -c --quiet --no-gitignore --no-ignore -d 0 -w '.trigger' -x 'run --package backend --bin stellarust --release';