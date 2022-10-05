#!/bin/bash

cargo watch -c --quiet -d 0 --no-gitignore --no-ignore  -w frontend/src/  -s '(cd frontend/; npm run lint); if [ $? -eq 0 ];  then rm -r target/*/build/frontend_static_files*; echo PASS; touch .trigger; else echo FAIL; fi;';