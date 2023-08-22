#!/bin/bash
surreal start --log debug --user root --pass root memory &
surreal isready  --conn http://localhost:8000
ls