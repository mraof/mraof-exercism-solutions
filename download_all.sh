#!/bin/sh
curl -LS https://exercism.io/tracks/$1/exercises | grep "/tracks/$1/exercises/" | awk '{print $3}' | cut -d/ -f5 | cut -d\" -f1 > exercises.txt
while read in; do exercism download --exercise="$in" --track=$1; done < exercises.txt
