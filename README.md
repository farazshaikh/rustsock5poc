Simple Socks Proxy.

POC for transit component for BTC integration. 

1. A simple python implementation of a SOCKS6 server that implments following minimal functionality
   - no auth
   - TCP connect
   - dispatch loop to relay data across the proxy

2. An echo server 
    Replies back with the data it receives.

3. Tokio socks server
     The tokio client connects to the echo server via the itermediate python SOCK proxy



     Tokio <->  PythonProxy <-> EchoServer


Steps to run.
1. Run the proxy it listens on the port 9011
   python ./server.py   

2. Run the server it listens on 3333

3. Run the client it connects to python socks proxy and talks to the echo server


Compiling Make targets

make buildenv

make 

make clean
