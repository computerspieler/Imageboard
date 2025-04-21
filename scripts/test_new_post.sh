#!/bin/sh

curl -i \
    --header "Content-Type: application/json" \
    --request POST \
    --data @$1 \
    http://localhost:3000/api/0/0/new
