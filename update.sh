#!/bin/bash
scp target/aarch64-unknown-linux-gnu/debug/$1 Developer@169.254.24.24:/home/Developer/
ssh Developer@169.254.24.24
