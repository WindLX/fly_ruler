# Lua Runtime

Some lua binding for some fly_ruler structs.

# API

## `CommandWrapper: Userdata`

### Fields

#### `type: String`

`Control`, `Extra` or `Exit`

#### `value: Table | String`

`Control` or `String`

## `OutputReceiverWrapper: Userdata`

### Methods

#### `async changed()`

Block until new data received.

#### `has_changed() -> Bool`

Return true if new data received.

#### `get() -> Table`

Receive output data. Fields:
- `time`: `Number`, `f64`;
- `data`: `Table`, `CoreOutput`.

#### `get_and_update() -> Table`

Receive output data and update channel state. Fields:
- `time`: `Number`, `f64`;
- `data`: `Table`, `CoreOutput`.

#### `clone() -> Userdata(OutputReceiverWrapper)`

Clone the receiver.

## `InputSenderWrapper: Userdata`

### Methods

#### `async send(command: Userdata(Command) | Nil)`

Send command to the plane which is connected.

## `UuidWrapper: Userdata`

Uuid

### Methods

#### `__tostring()`

Return uuid string.

#### `__eq(other: Userdata(UuidWrapper))`

Return true if uuid equal.