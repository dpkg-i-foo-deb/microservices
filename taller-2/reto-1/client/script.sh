#!/bin/bash
surreal isready --conn http://contenedor-bd-reto-1:8000

cat data/0-db.surql | surreal sql --conn http://contenedor-bd-reto-1:8000 --user root --pass root 

surreal import --conn http://contenedor-bd-reto-1:8000 --user root --pass root --db auth --ns auth data/1-model.surql