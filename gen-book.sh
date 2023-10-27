#!/bin/bash

# Store current directory
initialDir=$(pwd)

# Change directory to home
cd ~

# Run command
git ls-tree -r --name-only HEAD --full-tree src | xargs -I {} git log -1 --format="%aI,book/{}" {} | grep "\.rs$" | awk -F "." '{print $1}'

# Change back to initial directory 
cd "$initialDir"
