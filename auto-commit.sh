#!/bin/sh
BASEDIR=$(dirname "$0")
cd "$BASEDIR"
# Automatically generate the commit message,
# assuming that all changed files are added, not modified.
git commit -m "Add "$(git diff --name-only --cached)
