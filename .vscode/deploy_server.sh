#!/bin/bash
cd $1;
systemfd --no-pid -s http::$2 -- \
cargo-watch \
--ignore 'frontend_client/*' \
--ignore '.vscode/*' \
--ignore '.git/*' \
-x 'run --package actix_server --bin actix_server_bin --release';