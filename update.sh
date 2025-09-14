#!/bin/bash
scp target/aarch64-unknown-linux-gnu/debug/$1 Developer@192.168.1.104:/home/Developer/
ssh Developer@192.168.1.104
