# Json Viewer

The lib helps to convert collections of `Coreoutput` which received from `OutputReceiverWrapper` which from lua_runtime lib to json.

# API

## Function

### `encode(view_msg: Table) -> String`

Convert collections of `CoreOutput` to json, return the serialized result.

`view_msg` need to own the fields list below:

- `time`: `Number`. Actually, it's based on the `f64` type of Rust at the bottom. It will be provided by the return value of `receive` function of `OutputReceiverWrapper`.

- `view_msg`: `Table`. Due to the return value of `receive` function of `OutputReceiverWrapper` will only provide the data on one aircraft, it is necessary to manually assemble this data. It's a vec of `Table` which each `Table` contains the following fields:

    - `id`: `String`. Id of the plane.

    - `output`: `Table`. `CoreOutput`'s serialized result.

### `decode_cmd(cmd: String) -> Userdata(CommandWrapper)`

Convert json of command to userdata.

Like:
```json
{
    {
        "Control":
        {
            "thrust": 5000.0,
            "elevator": -0.09,
            "aileron": 0.01,
            "rudder": -0.01
        }
    },
    { "Extra": "String" },
    { "Exit" },
}
```