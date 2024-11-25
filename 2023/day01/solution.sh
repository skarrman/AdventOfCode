#!/bin/bash

pat='[0-9]'
rev_pat='[0-9]'

if [[ "$part" = "part2" ]]
then
    pat='([0-9]|one|two|three|four|five|six|seven|eight|nine)'
    rev_pat='([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)'
fi

> numbers
for row in $(cat input.txt)
do
 fst=$(echo $row | grep -o -E $pat | head -n 1 | sed -r -e s/one/1/g -e s/two/2/g -e s/three/3/g -e s/four/4/g -e s/five/5/g -e s/six/6/g -e s/seven/7/g -e s/eight/8/g -e s/nine/9/g)
 snd=$(echo $row | rev | grep -o -E $rev_pat | head -n 1 | sed -r -e s/eno/1/g -e s/owt/2/g -e s/eerht/3/g -e s/ruof/4/g -e s/evif/5/g -e s/xis/6/g -e s/neves/7/g -e s/thgie/8/g -e s/enin/9/g)
 echo "$fst$snd" >> numbers 
done

cat numbers | awk '{s+=$1} END {print s}'