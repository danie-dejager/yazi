use std::collections::{HashMap, VecDeque};

use mlua::Function;

#[derive(Debug, Default)]
pub struct Runtime {
	frames: VecDeque<RuntimeFrame>,
	blocks: HashMap<String, Vec<Function>>,
}

#[derive(Debug)]
struct RuntimeFrame {
	id:    String,
	calls: usize,
}

impl Runtime {
	pub fn new(id: &str) -> Self {
		Self {
			frames: VecDeque::from([RuntimeFrame { id: id.to_owned(), calls: 0 }]),
			..Default::default()
		}
	}

	pub fn push(&mut self, id: &str) {
		self.frames.push_back(RuntimeFrame { id: id.to_owned(), calls: 0 });
	}

	pub fn pop(&mut self) { self.frames.pop_back(); }

	pub fn current(&self) -> Option<&str> { self.frames.back().map(|f| f.id.as_str()) }

	pub fn current_owned(&self) -> Option<String> { self.current().map(ToOwned::to_owned) }

	pub fn next_block(&mut self) -> Option<usize> {
		self.frames.back_mut().map(|f| {
			f.calls += 1;
			f.calls - 1
		})
	}

	pub fn get_block(&self, id: &str, calls: usize) -> Option<Function> {
		self.blocks.get(id).and_then(|v| v.get(calls)).cloned()
	}

	pub fn put_block(&mut self, f: Function) -> bool {
		let Some(cur) = self.frames.back() else { return false };
		if let Some(v) = self.blocks.get_mut(&cur.id) {
			v.push(f);
		} else {
			self.blocks.insert(cur.id.to_owned(), vec![f]);
		}
		true
	}
}
