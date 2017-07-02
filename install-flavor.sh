#!/bin/bash

if [ -z "$1" ]; then
	echo "./install-flavor.sh <branch> [--merge] [config folder]"
	exit 1
fi

branch="$1"
dest="$2"
merge=false

if [ "$dest" == "--merge" ]; then
	dest="$3"
	merge=true
fi
if [ -z "$dest" ]; then
	dest=~/.config/insult
fi

if [ "$merge" == false ]; then
	echo "--merge not set."
	echo "Warning: You are about to completely replace your insult config files."
	echo -n "Continue? (y/n) "
	read -n 1 answer
	echo
	if [ "$answer" != "y" ]; then
		echo -n "Answer is not 'y'. Exitting!"
		exit 1
	fi

	rm -v "$dest/nouns"
	rm -v "$dest/endings"
	rm -v "$dest/verbs"

	if [ "$branch" == "master" ]; then
		echo "Executing insult to regenerate files..."
		(
			(which insult && insult) ||
			(stat ./target/release/insult && ./target/release/insult) ||
			find . -name insult -executable -type f -exec "{}" \; ||
			(echo "Can't guess where insult lives." && exit)
		) &> /dev/null
		exit
	fi
fi

if [ "$branch" == "master" ]; then
	echo "Branch master and --merge are not compatible. Exitting."
	exit
fi

git diff-index --quiet HEAD && has_changes=false || has_changes=true
if [ "$has_changes" == true ]; then
	git stash > /dev/null # In case of any uncommited changes
fi

git checkout "$branch" &> /dev/null

printf "\n# %s\n" "$branch" >> "$dest/nouns"
printf "\n# %s\n" "$branch" >> "$dest/endings"
printf "\n# %s\n" "$branch" >> "$dest/verbs"
cat nouns   >> "$dest/nouns"   && echo "\"nouns\"   -> \"$dest/nouns\""
cat endings >> "$dest/endings" && echo "\"endings\" -> \"$dest/endings\""
cat verbs   >> "$dest/verbs"   && echo "\"verbs\"   -> \"$dest/verbs\""

git checkout master &> /dev/null


if [ "$has_changes" == true ]; then
	git stash pop > /dev/null
fi
