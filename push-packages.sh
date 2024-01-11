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

    split_hash=`git splitsh --prefix "$1"`
    git push "$2" "$split_hash":main
}

push packages/php package-php
# push packages/rust package-rust
# push packages/typescript package-typescript