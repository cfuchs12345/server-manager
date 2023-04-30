#!/bin/sh

export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/


./server-manager-rust >> /var/log/server-manager-rust.log 2>&1