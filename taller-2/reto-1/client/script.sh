#!/bin/bash
surreal isready --conn http://contenedor-bd-reto-1:8000

ls

surreal import --conn http://contenedor-bd-reto-1:8000 --user root --pass root --db auth --ns auth /microservices/taller-2/reto-1/data/0-db.surql