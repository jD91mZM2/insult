#!/bin/bash

if [ -z "$1" ]; then
	echo "./install-flavor.sh <branch> [destination]"
	exit 1
fi

dest=~/.config/insult
if [ -n "$2" ]; then
	dest="$2"
fi

echo -n "You are about to replace your insult config files. Continue? (y/n)"
read -n 1 answer
echo
if [ "$answer" != "y" ]; then
	echo -n "Answer is not 'y'. Exitting!"
	exit 1
fi

if [ "$1" == "master" ]; then
	rm -v "$dest/nouns"
	rm -v "$dest/endings"
	rm -v "$dest/verbs"
	exit
fi

git checkout "$1" &> /dev/null
cp -v nouns "$dest/nouns"
cp -v endings "$dest/endings"
cp -v verbs "$dest/verbs"
git checkout master &> /dev/null
