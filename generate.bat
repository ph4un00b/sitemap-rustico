@echo off
chcp 65001 > nul
setlocal enabledelayedexpansion

call gen-articles.bat > gen-articles.txt
call gen-weekly.bat > gen-weekly.txt
call gen-book.bat > gen-book.txt
call gen-home.bat > gen-home.txt

cargo run --bin tags > gen-tags.txt

type gen-articles.txt gen-weekly.txt gen-book.txt gen-home.txt gen-tags.txt > dates_and_paths.txt

cargo run --bin generate