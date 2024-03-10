@echo off
chcp 65001 > nul
setlocal enabledelayedexpansion

call scripts/gen-dotnet.bat > generated/gen-dotnet.txt
call scripts/gen-articles.bat > generated/gen-articles.txt
call scripts/gen-weekly.bat > generated/gen-weekly.txt
call scripts/gen-book.bat > generated/gen-book.txt
call scripts/gen-home.bat > generated/gen-home.txt
cargo run --bin tags > generated/gen-tags.txt

:: Store current directory
set "initialDir=%CD%"

:: Change directory
cd /d generated

type gen-dotnet.txt gen-articles.txt gen-weekly.txt gen-book.txt gen-home.txt gen-tags.txt > dates_and_paths.txt
@REM type gen-dotnet.txt > dates_and_paths.txt

:: Change back to initial directory
cd /d !initialDir!

cargo run --bin generate
