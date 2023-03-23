# Echo server (no threads)

A simple implementation of an _echo server_ using tokio

Server : listens (via a socket) on some port,
and send back whatever content it recieves

Clients : connect to existing server, sends request and expects a response.
