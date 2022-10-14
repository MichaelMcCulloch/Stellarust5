#!/bin/bash
source /usr/share/nvm/init-nvm.sh;
cargo watch --use-shell=/bin/bash -c --quiet -d 0 --no-gitignore --no-ignore  -w frontend/src/  -s '(cd frontend/; npm run build); if [ $? -eq 0 ];  then rm -r target/*/build/frontend_static_files*; echo PASS; touch .trigger; else echo FAIL; fi;';