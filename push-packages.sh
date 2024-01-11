#!/usr/bin/env bash

git diff-index --quiet HEAD --

if [[ $? != 0 ]]; then
    echo
    echo "You have uncommitted changes. Commit and push them before running this script."
    echo

    exit
fi

branch=`git rev-parse --abbrev-ref HEAD`

if [[ "${branch}" != "main" ]]; then
    echo
    echo "You are not currently on the 'main' branch. There's no reason to push packages from any other branch."
    echo

    exit
fi


# Usage: push <subtree-path> <remote-name>
function push() {
    echo -n "Pushing $0 to $1... "

    split_hash=`git splitsh --prefix "$1" 2>/dev/null`
    git push "$2" "$split_hash":main

    echo "Done"
}

push packages/php package-php
# push packages/rust package-rust
# push packages/typescript package-typescript