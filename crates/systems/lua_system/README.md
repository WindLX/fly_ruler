# Lua System

The lua binding of FlyRuler Core. Async function must be wrapped in lua's coroutine to use.

# API

## Function

### `system.new() -> Userdata(System)`

Create a new `System` object.

### `command.control(control: Table) -> Userdata(Command)`

Create a new `Command` which carry `Control`.

`control`: must own fields below:

- `thrust`: `Number`;
- `elevator`: `Number`;
- `aileron`: `Number`;
- `rudder`: `Number`.

### `command.exit() -> Userdata(Command)`

Create a `Command::Exit` command.

### `command.extra(extra: String) -> Userdata(Command)`

Create a `Command::Extra` command.

### `command.default() -> Userdata(Command)`

Create a default `Control` command.

### `logger.init(settings: Table)`

Initialize logger.

`settings`: Init settings. It needs fields below:

- `target`: `String`. `Stdout` or `Stderr`.
- `timestamp`: `String`. `Millis`, `Nanos`, `Seconds` or `Micros`. Default to `Micros`. Whether to add timestamp or format.

### `logger.trace(message: String)`

Write trace log.

### `logger.debug(message: String)`

Write debug log.

### `logger.info(message: String)`

Write info log.

### `logger.warn(message: String)`

Write warn log.

### `logger.error(message: String)`

Write error log.

### `uuid_set.new() -> Userdata(UuidSet)`

Create a new `UuidSet` object.

### `uuid_set.uuid_v4() -> Userdata(UuidWrapper)`

Generate a new `UuidWrapper` object.

## `System: Userdata`

### Fields

#### `models: Table`

Vec of models which the system loads. Each element contains fields below:

- `info`: `Table`. From `PluginInfo`.
- `state`: `String`. From `PluginState`.

#### `planes: Table`

Vec of planes id which the system contains. Wrap `Vec<usize>`.

### Methods

#### `set_dir(path: String)`

Set the directory of the models for system.

#### `init(init_cfg: Table)`

Initialize the core in the system.

`init_cfg` needs fields below:
- `sample_time`: `Number` or `Nil`. `Option<u64>`;
- `time_scale`: `Number` or `Nil`. `Option<f64>`.

#### `async step()`

One step simulation.

#### `stop()`

Stop the system.

#### `enable_model(index: Number, args: Table)`

Enable the target model.

- `index`: Model id;
- `args`: `Vec<String>`, arguments for installing.

#### `disable_model(index: Number)`

Disable the target model.

`index`: Model id.

#### `get_model_state(index: Number) -> String | Nil`

Get the state of the target model. return `Option<PluginState>`.

`index`: Model id.

#### `set_controller(index: Number, buffer: Number) -> Userdata(InputSenderWrapper)`

Set a new controller for the target plane and get its sender.

- `index`: Plane id;
- `buffer`: Channel buffer length.

#### `async push_plane(index: Number, init_cfg: Table) -> Table`

Push a plane to the system and get its data receiver.

`index`: Id of the model.

`init_cfg`: `PlaneInitCfg`. It needs fields below:

- `deflection`: `Table`, `Option[f64; 3]`;
- `trim_target`: `Table`, `TrimTarget`:
    - `altitude`: `Number`, `f64`;
    - `velocity`: `Number`, `f64`.
- `trim_init`: `Table | Nil`, `Option<TrimInit>`:
    - `control`: `Table`, `Control`;
    - `alpha`: `Number`, `f64`.
- `flight_condition`: `String | Nil`, `Option<FlightCondition>`:    `WingsLevel`, `Turning`, `PullUp` or `Roll`;
- `optim_options`: `Table | Nil`, `Option<NelderMeadOptions>`:
    - `max_fun_evals`: `Number`, `usize`,
    - `max_iter`: `Number`, `usize`,
    - `tol_fun`: `Number`, `f64`,
    - `tol_x`: `Number`, `f64`,

Return `{ Userdata(UuidWrapper), Userdata(OutputReceiverWrapper) }`

#### `subscribe_plane(index: Number) -> Userdata(OutputReceiverWrapper)`

Get a new data receiver to target plane.

`index`: Id of the model.

#### `contain_plane(index: Number) -> Boolean`

Check if the system contains the target plane.

#### `async get_time() -> Number`

Get current time of system.

#### `async pause()`

Pause the system.

#### `async resume()`

Resume the system.

## `UuidSet: Userdata`

UuidSet of UuidWrapper.

### Methods

#### `__len()`

Return len.

#### `add(Userdata(UuidWrapper))`

Add a UuidWrapper to UuidSet.

#### `remove(Userdata(UuidWrapper))`

Remove a UuidWrapper from UuidSet.

#### `contains(Userdata(UuidWrapper)) -> Boolean`

Check if the UuidSet contains the UuidWrapper.

#### `to_table() -> Table`

Return a table of UuidWrapper.
