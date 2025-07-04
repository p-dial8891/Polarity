#!/bin/bash
link=$(ip link show | grep -io "enx[^:]*")
ip addr add 169.254.24.30/16 dev $link
ip link set $link up 
