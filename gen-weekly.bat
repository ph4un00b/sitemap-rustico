@echo off
setlocal enabledelayedexpansion

:: Store current directory
set "initialDir=%CD%"

:: Change directory to home
cd /d blog

:: Run command
git ls-tree -r --name-only HEAD --full-tree esta_semana_en_rust | xargs -I {} git log -1 --format="%%aI,blog/{}" {} | grep "\.md$" | awk -F "." "{print $1}"

:: Change back to initial directory
cd /d !initialDir!
