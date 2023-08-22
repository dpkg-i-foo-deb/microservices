#!/bin/bash
surreal start --log debug --user root --pass root memory &
surreal isready  --conn http://localhost:8000
surreal import --conn http://localhost:8000 --user root --pass root data/0-db.surql