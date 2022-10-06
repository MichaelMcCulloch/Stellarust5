Set-Location $args[0]

cargo watch -c --quiet -d 0 --no-gitignore -x 'check --all' -s ' type '' > .trigger ' 
