@echo off
setlocal enabledelayedexpansion

:: Store current directory
set "initialDir=%CD%"

:: Change directory to home
cd /d book

:: Run command
git ls-tree -r --name-only HEAD --full-tree src | xargs -I {} git log -1 --format="%%aI,book/{}" {} | grep "\.md$" | awk -F "." "{print $1}"

:: Change back to initial directory
cd /d !initialDir!
