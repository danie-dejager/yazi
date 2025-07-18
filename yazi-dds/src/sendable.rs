use std::{any::TypeId, borrow::Cow, collections::HashMap};

use mlua::{ExternalError, IntoLua, Lua, MultiValue, Table, Value};
use ordered_float::OrderedFloat;
use yazi_shared::{event::{Data, DataKey}, replace_cow};

pub struct Sendable;

impl Sendable {
	pub fn value_to_data(lua: &Lua, value: Value) -> mlua::Result<Data> {
		Ok(match value {
			Value::Nil => Data::Nil,
			Value::Boolean(b) => Data::Boolean(b),
			Value::LightUserData(_) => Err("light userdata is not supported".into_lua_err())?,
			Value::Integer(i) => Data::Integer(i),
			Value::Number(n) => Data::Number(n),
			Value::String(b) => {
				if let Ok(s) = b.to_str() {
					Data::String(s.to_owned().into())
				} else {
					Data::Bytes(b.as_bytes().to_owned())
				}
			}
			Value::Table(t) => {
				let (mut i, mut map) = (1, HashMap::with_capacity(t.raw_len()));
				for result in t.pairs::<Value, Value>() {
					let (k, v) = result?;
					let k = Self::value_to_key(k)?;

					if k == DataKey::Integer(i) {
						i += 1;
					}
					map.insert(k, Self::value_to_data(lua, v)?);
				}

				if map.len() == i as usize - 1 {
					Data::List((1..i).map(|i| map.remove(&DataKey::Integer(i)).unwrap()).collect())
				} else {
					Data::Dict(map)
				}
			}
			Value::Function(_) => Err("function is not supported".into_lua_err())?,
			Value::Thread(_) => Err("thread is not supported".into_lua_err())?,
			Value::UserData(ud) => match ud.type_id() {
				Some(t) if t == TypeId::of::<yazi_binding::Url>() => {
					Data::Url(ud.take::<yazi_binding::Url>()?.into())
				}
				Some(t) if t == TypeId::of::<yazi_binding::Urn>() => {
					Data::Urn(ud.take::<yazi_binding::Urn>()?.into())
				}
				Some(t) if t == TypeId::of::<yazi_binding::Id>() => {
					Data::Id(**ud.borrow::<yazi_binding::Id>()?)
				}
				Some(t) if t == TypeId::of::<yazi_fs::FilesOp>() => {
					Data::Any(Box::new(ud.take::<yazi_fs::FilesOp>()?))
				}
				Some(t) if t == TypeId::of::<yazi_parser::mgr::UpdateYankedIter>() => {
					Data::Any(Box::new(ud.take::<yazi_parser::mgr::UpdateYankedIter>()?.into_opt(lua)?))
				}
				_ => Err(format!("unsupported userdata included: {ud:?}").into_lua_err())?,
			},
			Value::Error(_) => Err("error is not supported".into_lua_err())?,
			Value::Other(..) => Err("unknown data is not supported".into_lua_err())?,
		})
	}

	pub fn data_to_value(lua: &Lua, data: Data) -> mlua::Result<Value> {
		Ok(match data {
			Data::List(l) => {
				let mut vec = Vec::with_capacity(l.len());
				for v in l.into_iter() {
					vec.push(Self::data_to_value(lua, v)?);
				}
				Value::Table(lua.create_sequence_from(vec)?)
			}
			Data::Dict(d) => {
				let seq_len = d.keys().filter(|&k| k.is_integer()).count();
				let tbl = lua.create_table_with_capacity(seq_len, d.len() - seq_len)?;
				for (k, v) in d {
					tbl.raw_set(Self::key_to_value(lua, k)?, Self::data_to_value(lua, v)?)?;
				}
				Value::Table(tbl)
			}
			Data::Url(u) => yazi_binding::Url::new(u).into_lua(lua)?,
			Data::Urn(u) => yazi_binding::Urn::new(u).into_lua(lua)?,
			Data::Any(a) => {
				if a.is::<yazi_fs::FilesOp>() {
					lua.create_any_userdata(*a.downcast::<yazi_fs::FilesOp>().unwrap())?.into_lua(lua)?
				} else if a.is::<yazi_parser::mgr::UpdateYankedOpt>() {
					a.downcast::<yazi_parser::mgr::UpdateYankedOpt>().unwrap().into_lua(lua)?
				} else {
					Err("unsupported Data::Any included".into_lua_err())?
				}
			}
			data => Self::data_to_value_ref(lua, &data)?,
		})
	}

	pub fn data_to_value_ref(lua: &Lua, data: &Data) -> mlua::Result<Value> {
		Ok(match data {
			Data::Nil => Value::Nil,
			Data::Boolean(b) => Value::Boolean(*b),
			Data::Integer(i) => Value::Integer(*i),
			Data::Number(n) => Value::Number(*n),
			Data::String(s) => Value::String(lua.create_string(s.as_ref())?),
			Data::List(l) => {
				let mut vec = Vec::with_capacity(l.len());
				for v in l {
					vec.push(Self::data_to_value_ref(lua, v)?);
				}
				Value::Table(lua.create_sequence_from(vec)?)
			}
			Data::Dict(d) => {
				let seq_len = d.keys().filter(|&k| k.is_integer()).count();
				let tbl = lua.create_table_with_capacity(seq_len, d.len() - seq_len)?;
				for (k, v) in d {
					tbl.raw_set(Self::key_to_value_ref(lua, k)?, Self::data_to_value_ref(lua, v)?)?;
				}
				Value::Table(tbl)
			}
			Data::Id(i) => yazi_binding::Id(*i).into_lua(lua)?,
			Data::Url(u) => yazi_binding::Url::new(u.clone()).into_lua(lua)?,
			Data::Urn(u) => yazi_binding::Urn::new(u.clone()).into_lua(lua)?,
			Data::Bytes(b) => Value::String(lua.create_string(b)?),
			Data::Any(a) => {
				if let Some(t) = a.downcast_ref::<yazi_fs::FilesOp>() {
					lua.create_any_userdata(t.clone())?.into_lua(lua)?
				} else if let Some(t) = a.downcast_ref::<yazi_parser::mgr::UpdateYankedOpt>() {
					t.clone().into_lua(lua)?
				} else {
					Err("unsupported Data::Any included".into_lua_err())?
				}
			}
		})
	}

	pub fn table_to_args(lua: &Lua, t: Table) -> mlua::Result<HashMap<DataKey, Data>> {
		let mut args = HashMap::with_capacity(t.raw_len());
		for pair in t.pairs::<Value, Value>() {
			let (k, v) = pair?;
			match k {
				Value::Integer(i) if i > 0 => {
					args.insert(DataKey::Integer(i - 1), Self::value_to_data(lua, v)?);
				}
				Value::String(s) => {
					args.insert(
						DataKey::String(Cow::Owned(s.to_str()?.replace('_', "-"))),
						Self::value_to_data(lua, v)?,
					);
				}
				_ => return Err("invalid key in Cmd".into_lua_err()),
			}
		}
		Ok(args)
	}

	pub fn args_to_table(lua: &Lua, args: HashMap<DataKey, Data>) -> mlua::Result<Table> {
		let seq_len = args.keys().filter(|&k| k.is_integer()).count();
		let tbl = lua.create_table_with_capacity(seq_len, args.len() - seq_len)?;
		for (k, v) in args {
			match k {
				DataKey::Integer(i) => tbl.raw_set(i + 1, Self::data_to_value(lua, v)?),
				DataKey::String(s) => tbl.raw_set(replace_cow(s, "-", "_"), Self::data_to_value(lua, v)?),
				_ => Err("invalid key in Data".into_lua_err()),
			}?;
		}
		Ok(tbl)
	}

	pub fn args_to_table_ref(lua: &Lua, args: &HashMap<DataKey, Data>) -> mlua::Result<Table> {
		let seq_len = args.keys().filter(|&k| k.is_integer()).count();
		let tbl = lua.create_table_with_capacity(seq_len, args.len() - seq_len)?;
		for (k, v) in args {
			match k {
				DataKey::Integer(i) => tbl.raw_set(i + 1, Self::data_to_value_ref(lua, v)?),
				DataKey::String(s) => {
					tbl.raw_set(replace_cow(s.as_ref(), "-", "_"), Self::data_to_value_ref(lua, v)?)
				}
				_ => Err("invalid key in Data".into_lua_err()),
			}?;
		}
		Ok(tbl)
	}

	pub fn list_to_values(lua: &Lua, data: Vec<Data>) -> mlua::Result<MultiValue> {
		data.into_iter().map(|d| Self::data_to_value(lua, d)).collect()
	}

	pub fn values_to_list(lua: &Lua, values: MultiValue) -> mlua::Result<Vec<Data>> {
		values.into_iter().map(|v| Self::value_to_data(lua, v)).collect()
	}
}

impl Sendable {
	fn value_to_key(value: Value) -> mlua::Result<DataKey> {
		Ok(match value {
			Value::Nil => DataKey::Nil,
			Value::Boolean(b) => DataKey::Boolean(b),
			Value::LightUserData(_) => Err("light userdata is not supported".into_lua_err())?,
			Value::Integer(i) => DataKey::Integer(i),
			Value::Number(n) => DataKey::Number(OrderedFloat(n)),
			Value::String(s) => {
				if let Ok(s) = s.to_str() {
					DataKey::String(s.to_owned().into())
				} else {
					DataKey::Bytes(s.as_bytes().to_owned())
				}
			}
			Value::Table(_) => Err("table is not supported".into_lua_err())?,
			Value::Function(_) => Err("function is not supported".into_lua_err())?,
			Value::Thread(_) => Err("thread is not supported".into_lua_err())?,
			Value::UserData(ud) => match ud.type_id() {
				Some(t) if t == TypeId::of::<yazi_binding::Url>() => {
					DataKey::Url(ud.take::<yazi_binding::Url>()?.into())
				}
				Some(t) if t == TypeId::of::<yazi_binding::Urn>() => {
					DataKey::Urn(ud.take::<yazi_binding::Urn>()?.into())
				}
				Some(t) if t == TypeId::of::<yazi_binding::Id>() => {
					DataKey::Id(**ud.borrow::<yazi_binding::Id>()?)
				}
				_ => Err(format!("unsupported userdata included: {ud:?}").into_lua_err())?,
			},
			Value::Error(_) => Err("error is not supported".into_lua_err())?,
			Value::Other(..) => Err("unknown data is not supported".into_lua_err())?,
		})
	}

	#[inline]
	fn key_to_value(lua: &Lua, key: DataKey) -> mlua::Result<Value> {
		match key {
			DataKey::Url(u) => yazi_binding::Url::new(u).into_lua(lua),
			DataKey::Urn(u) => yazi_binding::Urn::new(u).into_lua(lua),
			_ => Self::key_to_value_ref(lua, &key),
		}
	}

	fn key_to_value_ref(lua: &Lua, key: &DataKey) -> mlua::Result<Value> {
		Ok(match key {
			DataKey::Nil => Value::Nil,
			DataKey::Boolean(b) => Value::Boolean(*b),
			DataKey::Integer(i) => Value::Integer(*i),
			DataKey::Number(n) => Value::Number(n.0),
			DataKey::String(s) => Value::String(lua.create_string(s.as_ref())?),
			DataKey::Id(i) => yazi_binding::Id(*i).into_lua(lua)?,
			DataKey::Url(u) => yazi_binding::Url::new(u.clone()).into_lua(lua)?,
			DataKey::Urn(u) => yazi_binding::Urn::new(u.clone()).into_lua(lua)?,
			DataKey::Bytes(b) => Value::String(lua.create_string(b)?),
		})
	}
}
