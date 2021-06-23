#!/bin/bash

cd 1.basic

for D in *; do
    if [ -d "${D}" ]; then
        echo Testing ${D}
        cd ${D}/solution/tests && npm install && npm test
        cd ../../..
    fi
done

cd ../2.intermediate

for D in *; do
    if [ -d "${D}" ]; then
        echo Testing ${D}
        cd ${D}/solution/tests && npm install && npm test
        cd ../../..
    fi
done
