#Comment


cargo clean

Get-ChildItem -Path ./coverage_prof -Recurse | Where-Object { $_.Name -ne ".gitkeep" } | Remove-Item -Recurse -Force
Get-ChildItem -Path ./coverage -Recurse | Where-Object { $_.Name -ne ".gitkeep" } | Remove-Item -Recurse -Force
