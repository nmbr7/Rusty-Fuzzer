#!/bin/sh

#Script to build and run rusty-fuzzer as an example on the `fuzzgoat` vulnerable json parser
#Fuzzgoat [ https://github.com/fuzzstati0n/fuzzgoat.git ]


FUZZER='../../target/debug/rusty-fuzzer'
COMPILER='../../Build'
CR='\e[m\e[1;31m'
CG='\e[m\e[1;32m'
CB='\e[m\e[1;7;39m'
CE='\e[m'

echo -e "$CG[#]$CR Building Fuzzer$CE"
cargo build
echo -e "$CG[#]$CR Cloning fuzzgoat$CE"
git clone https://github.com/fuzzstati0n/fuzzgoat.git 
pushd fuzzgoat >/dev/null
fuzzg=$PWD
pushd $COMPILER >/dev/null  
echo -e "$CG[#]$CR Compiling fuzzgoat$CE"
. ./compile $fuzzg'/fuzzgoat.c' $fuzzg'/main.c'
popd >/dev/null
cp $COMPILER'/a.out' .
rm $COMPILER'/a.out'
mkdir seeds
mv seed seeds
echo -e "$CG[#]$CR Executing fuzzer$CE"
echo -e "$CG[#]$CR Running Command $CB rusty-fuzzer -i 'a.out @' -s seeds -t f$CE"
sleep 3

$FUZZER -i "a.out @" -s seeds -t f
