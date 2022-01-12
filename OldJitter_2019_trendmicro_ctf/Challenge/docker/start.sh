#!/bin/sh
# Add your startup script
cd /home/ctf;
socat tcp-listen:9999,reuseaddr,fork exec:./ch,su=ctf;
while true
do
 sleep 2
 /myTmp
done
