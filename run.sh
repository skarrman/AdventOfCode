#!/bin/bash
path=$1

clang++ -std=c++11 -stdlib=libc++ -Weverything  -Wno-c++98-compat $path/*.cpp utils/*.cpp
echo "-------------------------------------------------------------------------------"
time ./a.out
echo ""
echo "-------------------------------------------------------------------------------"
rm -rf a.out