#!/bin/sh

# ./install-flavor-all [config folder]
./install-flavor.sh master "$1"
./install-flavor.sh flavor-arrowverse --merge "$1"
./install-flavor.sh flavor-meme --merge "$1"
