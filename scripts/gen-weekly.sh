#!/bin/bash

# Store current directory
initialDir=$(pwd)

# Change directory to home
cd blog

# Run command
git ls-tree -r --name-only HEAD --full-tree esta_semana_en_rust | xargs -I {} git log -1 --format="%aI,blog/{}" {} | grep "\.md$" | awk -F "." '{print $1}'

# Change back to initial directory
cd "$initialDir"
