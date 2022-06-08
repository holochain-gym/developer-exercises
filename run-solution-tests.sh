#!/bin/bash
set -e

cd 1.basic

for D in *; do
    if [ -d "${D}" ]; then
        if [ -d "${D}/solution/tests" ]; then
            echo Testing ${D}
            cd ${D}/solution/tests && npm test
            cd ../../..
        fi
    fi
done

cd ../2.intermediate

for D in *; do
    if [ -d "${D}" ]; then
        if [ -d "${D}/solution/tests" ]; then
            echo Testing ${D}
            cd ${D}/solution/tests && npm test
            cd ../../..
        fi
    fi
done
