#Comment

param(
    [String]$p,
    [String]$m
)

cargo clean

Get-ChildItem -Path ./coverage_prof -Recurse | Where-Object { $_.Name -ne ".gitkeep" } | Remove-Item -Recurse -Force
Get-ChildItem -Path ./coverage -Recurse | Where-Object { $_.Name -ne ".gitkeep" } | Remove-Item -Recurse -Force
Write-Host "Start coverage"

$toolchain = rustup default

if ($toolchain -eq "nightly-x86_64-pc-windows-msvc (default)")
{
    $env:RUSTFLAGS = "-Cinstrument-coverage"
}
elseif ($toolchain -eq "my-nightly (default)")
{
    $env:RUSTFLAGS = "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
}
else
{
    throw "Err:Unexpected toolchain."
}

if ($p)
{
    Write-Host ("Package :{0}" -f $p)
}

if ($m)
{
    Write-Host ("Module :{0}" -f $m)
}

$env:CARGO_INCREMENTAL = 0
$env:LLVM_PROFILE_FILE = "../coverage_prof/traq-bot-http-rs-%p-%m.profraw"


if ($p -and $m)
{
    $cmd = "cargo test $m -p $p"
    Write-Host("Command :{0}" -f $cmd)
    Invoke-Expression($cmd)

    #    cargo test $m -p $p
}
elseif($p)
{
    cargo test -p $p
}
else
{
    cargo test
}


grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/


Remove-Item ENV:CARGO_INCREMENTAL
Remove-Item ENV:RUSTFLAGS
Remove-Item ENV:LLVM_PROFILE_FILE

Start-Process "./coverage/html/index.html"
