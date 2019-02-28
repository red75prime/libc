#!/bin/sh
set -e
sshpass -p 'admin' scp $1 root@192.168.1.103:/tmp
sshpass -p 'admin' ssh root@192.168.1.103 /tmp/`basename $1`
