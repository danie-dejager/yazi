use mlua::{ExternalError, IntoLua, Lua, Value};
use yazi_shared::event::{CmdCow, Data};

#[derive(Debug)]
pub struct SeekOpt {
	pub units: i16,
}

impl From<CmdCow> for SeekOpt {
	fn from(c: CmdCow) -> Self { Self { units: c.first().and_then(Data::as_i16).unwrap_or(0) } }
}

impl IntoLua for &SeekOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
