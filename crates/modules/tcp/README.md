# Tcp

The lib provides basic methods to create a async tcp server and client. All async function must be wrapped in lua's coroutine to use.

# API

## Function

### `async connect(addr: String) -> Userdata(LuaTcpStream)`

Connect to a tcp server to create a tcp client.

The `addr` must be the format of `host:port`.

### `async bind(addr: String) -> Userdata(LuaTcpListener)`

Bind to a `addr` to create a tcp server.

The `addr` must be the format of `host:port`.

## `TcpStream: Userdata`

### Methods

#### `async write(data: String)`

Write `data` to stream. `String`: array of `u8`.

#### `async read() -> String`

Read data from stream. `String`: array of `u8`.

#### `async writable()`

Wait for the stream is writable.

#### `async readable()`

Wait for the stream is readable.

## `TcpListener: Userdata`

### Methods

#### `async accept() -> (Userdata(LuaTcpStream), String)`

Accept a new client.

Return the writable and readable stream of the client and the address of the new client.