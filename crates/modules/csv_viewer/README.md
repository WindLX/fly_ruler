# CSV Viewer

The lib helps to write `Coreoutput` which received from `OutputReceiverWrapper` which from lua_runtime lib into csv file.

# API

## Function

### `new(path: String) -> Userdata(CSVViewer)`

Build a new `CSVViewer`. The target `path` of csv file which will be created if not exist.

## `CSVViewer: Userdata`

### Methods

#### `write_line(data: Table)`

Write a line of csv file.

`data` need to contain the following fields:

- `time`: `Number`. Actually, it's based on the `f64` type of Rust at the bottom. It will be provided by the return value of `receive` function of `OutputReceiverWrapper`.

- `output`: `Table`. `CoreOutput`'s serialized to `Table` result. It will be provided by the return value of `receive` function of `OutputReceiverWrapper`.