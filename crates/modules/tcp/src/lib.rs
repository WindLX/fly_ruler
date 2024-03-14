use lazy_static::lazy_static;
use mlua::prelude::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

lazy_static! {
    static ref RT: tokio::runtime::Runtime = {
        std::thread::spawn(|| RT.block_on(futures::future::pending::<()>()));
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    };
    static ref GUARD: tokio::runtime::EnterGuard<'static> = RT.enter();
}

struct LuaTcpListener {
    listener: TcpListener,
}

impl LuaTcpListener {
    async fn bind(addr: &str) -> LuaResult<Self> {
        let listener = TcpListener::bind(addr).await;
        match listener {
            Ok(listener) => Ok(Self { listener }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    async fn accept(&self) -> LuaResult<(LuaTcpStream, String)> {
        let (stream, addr) = self.listener.accept().await?;
        Ok((LuaTcpStream { stream }, addr.to_string()))
    }
}

impl LuaUserData for LuaTcpListener {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut(
            "accept",
            |_lua, this, ()| async move { this.accept().await },
        );
    }
}

struct LuaTcpStream {
    stream: TcpStream,
}

impl LuaTcpStream {
    async fn connect(addr: &str) -> LuaResult<Self> {
        let stream = TcpStream::connect(addr).await;
        match stream {
            Ok(stream) => Ok(Self { stream }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    async fn write(&mut self, data: &str) -> LuaResult<()> {
        self.stream.write_all(data.as_bytes()).await?;
        Ok(())
    }

    async fn read(&mut self) -> LuaResult<String> {
        let mut buffer = Vec::new();
        let n = self.stream.read_buf(&mut buffer).await?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}

impl LuaUserData for LuaTcpStream {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("write", |_lua, this, data: String| async move {
            this.write(&data).await
        });
        methods.add_async_method_mut("read", |_lua, this, ()| async move { this.read().await });
    }
}

async fn bind<'lua>(_lua: &'lua Lua, addr: mlua::String<'lua>) -> LuaResult<LuaTcpListener> {
    LuaTcpListener::bind(addr.to_str().unwrap()).await
}

async fn connect<'lua>(_lua: &'lua Lua, addr: mlua::String<'lua>) -> LuaResult<LuaTcpStream> {
    LuaTcpStream::connect(addr.to_str().unwrap()).await
}

#[mlua::lua_module]
fn tcp(lua: &Lua) -> LuaResult<LuaTable> {
    let _guard = &*GUARD;
    let exports = lua.create_table()?;
    let tcp_stream = lua.create_table()?;
    tcp_stream.set("connect", lua.create_async_function(connect)?)?;

    let tcp_listener = lua.create_table()?;
    tcp_listener.set("bind", lua.create_async_function(bind)?)?;

    exports.set("tcp_stream", mlua::Value::Table(tcp_stream))?;
    exports.set("tcp_listener", mlua::Value::Table(tcp_listener))?;
    Ok(exports)
}
