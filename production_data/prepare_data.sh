#!/bin/bash

tar -xf production_data.tar.xz
for version in "3.4.5.95132" "3.5.1.98532"
do
    (
        cd ./$version;
        for campaign in ./*/
        do
            (
                cd ./$campaign
                for save in ./*
                do
                    (
                        cd ./$save
                        zip -r ../$save.sav ./meta ./gamestate
                    )
                    rm -r $save
                done
            )
        done
    )
    
done
