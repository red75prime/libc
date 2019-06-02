#!/bin/sh
set -e
sshpass -p 'hisi' scp $1 root@192.168.1.108:/tmp
sshpass -p 'hisi' ssh root@192.168.1.108 /tmp/`basename $1`
