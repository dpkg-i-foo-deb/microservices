#!/bin/bash
surreal start --log debug --user root --pass root memory &
surreal isready  --conn http://localhost:8000
cat data/0-db.surql | surreal sql --conn http://localhost:8000 --user root --pass root 
