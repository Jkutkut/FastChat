# FastChat

## Tutorial:
To communicate with websockets, use the endpoint: `ws://localhost:4242/ws/{uuid}`. The uuid is unique for each user.

## Tools used:
- **Android Studio**: To build the Android app.
- **Rust**: To build the server.
- **Docker**: To be able to run the server on any machine.
- **Makefile**: To automate the processes.

## Docker build server:
Builds the server image. Only needs to be run once.
```bash
make server
```

## Docker run server:
Runs the server on localhost.
```bash
make run-server
```

## Docker stop server:
Stops the current running server.
```bash
make stop-server
```

## Debug functions:
### Add user:
```bash
make addPaco
```

```bash
make addPepe
```
