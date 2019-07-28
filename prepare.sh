#!/usr/bin/env bash

git submodule update --init &&
cd frontend &&
./build-and-copy-frontend.sh &&
cd ../topology && lein install && cd .. &&
lein modules uberjar &&
mvn -f command-handler clean package &&
./create-certs.sh &&
java -jar test/target/test.jar mapping &&
docker-compose -f docker-bank.yml -f docker-prep.yml build