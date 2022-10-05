#!/bin/bash

cargo watch -c --quiet -d 0 --no-gitignore -x 'check --all' -s 'touch .trigger'