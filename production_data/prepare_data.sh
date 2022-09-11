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
                (
                    cd $save
                    zip -r ../$save.sav ./meta ./gamestate
                    
                )
                rm -r $save
                
            done
            
        )
    done
)