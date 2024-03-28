# Lua System

The lua binding of FlyRuler Core. Async function must be wrapped in lua's coroutine to use.

# API

## Function

### `system.new() -> Userdata(SystemWrapper)`

Create a new `SystemWrapper` object.

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

### `uuid.new_v4() -> Userdata(UuidWrapper)`

Generate a new `UuidWrapper` object.

### `uuid.parse_str(str: String) -> Userdata(UuidWrapper)`

Parse a `UuidWrapper` object from `String`.

## `SystemWrapper: Userdata`

### Fields

#### `models: Table`

Map of models which the system loads. Each element contains fields below:

- `info`: `Table`. From `PluginInfo`.
- `state`: `String`. From `PluginState`.

#### `planes: Table`

Vec of planes id which the system contains. Wrap `Vec<UuidWrapper>`.

### Methods

#### `set_dir(path: String)`

Set the directory of the models for system.

#### `init(init_cfg: Table)`

Initialize the core in the system.

`init_cfg` needs fields below:
- `sample_time`: `Number` or `Nil`. `Option<u64>`;
- `time_scale`: `Number` or `Nil`. `Option<f64>`.

#### `async step(is_block: bool)`

One step simulation.

#### `stop()`

Stop the system.

#### `enable_model(model_id: Userdata(UuidWrapper), args: Table)`

Enable the target model.

- `model_id`: Model id;
- `args`: `Vec<String>`, arguments for installing.

#### `disable_model(model_id: Userdata(UuidWrapper))`

Disable the target model.

`model_id`: Model id.

#### `get_model_state(model_id: Userdata(UuidWrapper)) -> String | Nil`

Get the state of the target model. return `Option<PluginState>`.

`model_id`: Model id.

#### `async set_controller(model_id: Userdata(UuidWrapper), plane_id: Userdata(UuidWrapper) | Nil, buffer: Number) -> Userdata(InputSenderWrapper)`

Set a new controller for the target plane and get its sender.

- `model_id`: Model id;
- `plane_id`: Plane id;
- `buffer`: Channel buffer length.

#### `async push_plane(model_id: Userdata(UuidWrapper), init_cfg: Table) -> Table`

Push a plane to the system and get its data receiver.

`model_id`: Id of the model.

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

#### `async subscribe_plane(plane_id: Userdata(UuidWrapper)) -> Userdata(OutputReceiverWrapper)`

Get a new data receiver to target plane.

`plane_id`: Id of the model.

#### `async remove_plane(plane_id: Userdata(UuidWrapper))`

Remove the target plane from the system.

#### `contains_plane(plane_id: Userdata(UuidWrapper)) -> Boolean`

Check if the system contains the target plane.

#### `async get_time() -> Number`

Get current time of system.

#### `async pause()`

Pause the system.

#### `async resume()`

Resume the system.

#### `clone() -> Userdata(SystemWrapper)`

Clone the system.