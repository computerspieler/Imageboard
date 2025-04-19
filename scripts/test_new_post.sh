#!/bin/sh

curl -i \
    --header "Content-Type: application/json" \
    --request POST \
    --data @$1 \
    http://localhost:3000/0/0/new
