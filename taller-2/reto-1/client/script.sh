#!/bin/bash
surreal isready --conn http://contenedor-bd-reto-1:8000

surreal import --conn http://contenedor-bd-reto-1:8000 --user root --pass root --db auth ../data/0-db.surql