
Set-Location $args[0]

cargo watch --use-shell=powershell -c --quiet -d 0 --no-gitignore --no-ignore  -w .\\frontend\\src\\ -s 'cd frontend; npm run lint; if ($?) {cd .. ; Remove-Item -Path .\\target\\release\\build\\frontend_static_files-* -Recurse -Force -Confirm:$false ; Remove-Item -Path .\\target\\debug\\build\\frontend_static_files-* -Recurse -Force -Confirm:$false ; echo $null >>  .trigger } else { cd .. } ;' 