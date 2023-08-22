#!/bin/bash
surreal start --log debug --user root --pass root memory &
surreal isready  --connn http://localhost:8000
ls