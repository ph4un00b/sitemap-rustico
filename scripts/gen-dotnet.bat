@echo off
setlocal enabledelayedexpansion

:: Store current directory
set "initialDir=%CD%"

:: Change directory
cd /d dotnet

:: Run command
git ls-tree -r --name-only HEAD --full-tree src/es | xargs -I {} git log -1 --format="%%aI,dotnet/{}" {} | grep "\.md$" | awk -F "." "{print $1}"

:: Change back to initial directory
cd /d !initialDir!
