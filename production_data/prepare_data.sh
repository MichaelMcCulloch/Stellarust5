#!/bin/bash

tar -xf $1.tar.xz

cd $1
for dir in ./*
do
    cd $dir;
    zip -r ../$dir.sav ./*
    cd ..;
    rm -r $dir
done