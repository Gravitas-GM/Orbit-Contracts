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

# Usage: push <subtree-path> <remote-name> <remote-url>
function push() {
	local path="$1"
	local remote_name="$2"
	local remote_url="$3"

	git branch | grep "$remote_name" --quiet

	if [[ $? != 0 ]]; then
		read -p "The remote $remote_name does not exist; would you like to add it as $remote_url? [y/N] " -r

		if [[ "${REPLY,,}" != "y" ]]; then
			echo "Remote $remote_name not found and add denied, skipping push."
			echo

			return 1
		fi

		git remote add "$remote_name" "$remote_url"
	fi

    echo "Pushing $1 to $2... "

    split_hash=`git splitsh --prefix "$1" 2>/dev/null`
    git push "$2" "$split_hash":main

    echo
}

push packages/php package-php https://github.com/Gravitas-GM/Orbit-PHP-Contracts
# push packages/rust package-rust
# push packages/typescript package-typescript
