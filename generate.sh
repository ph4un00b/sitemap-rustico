#!/bin/bash

chmod +x scripts/gen-dotnet.sh
./scripts/gen-dotnet.sh > generated/gen-dotnet.txt

chmod +x scripts/gen-articles.sh
./scripts/gen-articles.sh > generated/gen-articles.txt

chmod +x scripts/gen-weekly.sh
./scripts/gen-weekly.sh > generated/gen-weekly.txt

chmod +x scripts/gen-home.sh
./scripts/gen-home.sh > generated/gen-home.txt

chmod +x scripts/gen-book.sh
./scripts/gen-book.sh > generated/gen-book.txt

cargo run --bin tags > generated/gen-tags.txt

cat generated/gen-dotnet.txt generated/gen-articles.txt generated/gen-weekly.txt generated/gen-book.txt generated/gen-home.txt generated/gen-tags.txt > generated/dates_and_paths.txt

# cat generated/dates_and_paths.txt
cargo run --bin generate
