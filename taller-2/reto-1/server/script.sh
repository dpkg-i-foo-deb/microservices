#!/bin/bash


if [ -d "database" ] 

then 

    surreal start --log debug --user root --pass root file:database/surreal.db &

    sleep 10

    #Schema
    surreal isready  --conn http://localhost:8000
    cat data/0-db.surql | surreal sql --conn http://localhost:8000 --user root --pass root
    surreal import --conn http://localhost:8000 --user root --pass root --db auth --ns auth data/1-model.surql

else

    surreal start --log debug --user root --pass root file:database/surreal.db

fi

