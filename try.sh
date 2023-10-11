#!/usr/bin/env bash


curl --location '127.0.0.1:8080/api/add' \
--header 'Content-Type: application/json' \
--data '{
    "name": "groceries"
}'

echo ""

curl --location '127.0.0.1:8080/api/add' \
--header 'Content-Type: application/json' \
--data '{
    "name": "clean"
}'
echo ""


curl --location '127.0.0.1:8080/api/add' \
--header 'Content-Type: application/json' \
--data '{
    "name": "exercise"
}'
echo ""

curl --location '127.0.0.1:8080/api/add' \
--header 'Content-Type: application/json' \
--data '{
    "name": "cook"
}'
echo ""

curl --location --request PATCH '127.0.0.1:8080/api/done/1'
echo ""

curl --location --request PATCH '127.0.0.1:8080/api/done/3'
echo ""

curl --location --request DELETE '127.0.0.1:8080/api/destroy/2'
echo ""


curl --location '127.0.0.1:8080/api/list' | jq
echo ""
