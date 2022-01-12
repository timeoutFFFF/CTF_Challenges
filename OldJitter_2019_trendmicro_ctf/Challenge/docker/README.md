## Build

```
docker build -t "jit1" 
```

DO NOT use *bin* as challenge's name

## Run

```
docker run -d -p "0.0.0.0:pub_port:9999" -h "jit1" --name="jit1" jit1
```

`pub_port` is the port you want to expose to the public network.

## Capture traffic

If you want to capture challenge traffic, just run `tcpdump` on the host. Here is an example.

```
tcpdump -w helloworld.pcap -i eth0 port pub_port
```
