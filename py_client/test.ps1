

$programPath = ".\.venv\Scripts\python.exe"
$programArgs = ".\main.py"

# 设置要启动的进程数量
$numberOfProcesses = 10

# 循环启动进程
for ($i = 1; $i -le $numberOfProcesses; $i++) {
    Start-Process -FilePath $programPath -ArgumentList $programArgs
    Write-Host "启动进程 $i"
    Start-Sleep -Seconds 3
}