# Protobuf Viewer

The lib helps to convert collections of `Coreoutput` which received from `OutputReceiverWrapper` which from lua_runtime lib to Protobuf.

# API

## Function

### `encode(view_msg: Table) -> String`

Convert collections of `CoreOutput` to Protobuf, return the serialized result.

`view_msg`. There are strict requirements on the table format. Refer to the proto file for details. It must own the fields list below:

- `time`: `Number`. Actually, it's based on the `f64` type of Rust at the bottom. It will be provided by the return value of `receive` function of `OutputReceiverWrapper`.

- `view_msg`: `Table`. Due to the return value of `receive` function of `OutputReceiverWrapper` will only provide the data on one aircraft, it is necessary to manually assemble this data. It's a vec of `Table` which each `Table` contains the two elements:

    1. `String`. Id of the plane.

    2. `Table`. `CoreOutput`'s serialized result.
