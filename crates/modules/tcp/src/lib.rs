use mlua::prelude::*;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

struct LuaTcpListener {
    listener: TcpListener,
}

impl LuaTcpListener {
    fn bind(addr: &str) -> LuaResult<Self> {
        let listener = TcpListener::bind(addr);
        match listener {
            Ok(listener) => Ok(Self { listener }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    fn accept(&self) -> LuaResult<(LuaTcpStream, String)> {
        let (stream, addr) = self.listener.accept()?;
        Ok((LuaTcpStream { stream }, addr.to_string()))
    }
}

impl LuaUserData for LuaTcpListener {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("accept", |_lua, this, ()| this.accept());
    }
}

struct LuaTcpStream {
    stream: TcpStream,
}

impl LuaTcpStream {
    fn connect(addr: &str) -> LuaResult<Self> {
        let stream = TcpStream::connect(addr);
        match stream {
            Ok(stream) => Ok(Self { stream }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    fn write(&mut self, data: &str) -> LuaResult<()> {
        self.stream.write_all(data.as_bytes())?;
        Ok(())
    }

    fn read(&mut self) -> LuaResult<String> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = Vec::new();
        let n = reader.read_until(b'\n', &mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}

impl LuaUserData for LuaTcpStream {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("write", |_lua, this, data: String| this.write(&data));
        methods.add_method_mut("read", |_lua, this, ()| this.read());
    }
}

fn bind<'lua>(_lua: &'lua Lua, addr: mlua::String<'lua>) -> LuaResult<LuaTcpListener> {
    LuaTcpListener::bind(addr.to_str().unwrap())
}

fn connect<'lua>(_lua: &'lua Lua, addr: mlua::String<'lua>) -> LuaResult<LuaTcpStream> {
    LuaTcpStream::connect(addr.to_str().unwrap())
}

#[mlua::lua_module]
fn tcp_server(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let tcp_stream = lua.create_table()?;
    tcp_stream.set("connect", lua.create_function(connect)?)?;

    let tcp_listener = lua.create_table()?;
    tcp_listener.set("bind", lua.create_function(bind)?)?;

    exports.set("tcp_stream", mlua::Value::Table(tcp_stream))?;
    exports.set("tcp_listener", mlua::Value::Table(tcp_listener))?;
    Ok(exports)
}
