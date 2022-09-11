#!/bin/bash

tar -xf $1.tar.xz
(
    cd $1
    for campaign in ./*
    do
        (
            cd $campaign
            for save in ./*
            do
                echo $save
                zip -r ./$save.sav $save/meta $save/gamestate
                rm -r $save
                
            done
            
        )
    done
)