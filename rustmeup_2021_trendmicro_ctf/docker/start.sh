cd /home/ctf;
socat tcp-listen:9999,reuseaddr,fork exec:./rustmeup,su=ctf;
