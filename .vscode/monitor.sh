#!/bin/bash
./.vscode/watch_frontend.sh &
./.vscode/watch_backend.sh &
./.vscode/watch_trigger.sh &

wait