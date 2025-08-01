use std::{ffi::OsString, mem, path::{MAIN_SEPARATOR_STR, Path, PathBuf}};

use anyhow::Result;
use yazi_fs::{CWD, expand_path, services::Local};
use yazi_macro::{act, render, succ};
use yazi_parser::cmp::{CmpItem, ShowOpt, TriggerOpt};
use yazi_proxy::CmpProxy;
use yazi_shared::{event::Data, natsort};

use crate::{Actor, Ctx};

pub struct Trigger;

impl Actor for Trigger {
	type Options = TriggerOpt;

	const NAME: &str = "trigger";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let cmp = &mut cx.cmp;
		if let Some(t) = opt.ticket {
			if t < cmp.ticket {
				succ!();
			}
			cmp.ticket = t;
		}

		let Some((parent, word)) = Self::split_path(&opt.word) else {
			return act!(cmp:close, cx, false);
		};

		if cmp.caches.contains_key(&parent) {
			let ticket = cmp.ticket;
			return act!(cmp:show, cx, ShowOpt { cache_name: parent, word: word.into(), ticket, ..Default::default() });
		}

		let ticket = cmp.ticket;
		tokio::spawn(async move {
			// TODO: support VFS
			let mut dir = Local::read_dir(&parent).await?;
			let mut cache = vec![];

			// "/" is both a directory separator and the root directory per se
			// As there's no parent directory for the FS root, it is a special case
			if parent == Path::new("/") {
				cache.push(CmpItem { name: OsString::new(), is_dir: true });
			}

			while let Ok(Some(ent)) = dir.next_entry().await {
				if let Ok(ft) = ent.file_type().await {
					cache.push(CmpItem { name: ent.file_name(), is_dir: ft.is_dir() });
				}
			}

			if !cache.is_empty() {
				cache.sort_unstable_by(|a, b| {
					natsort(a.name.as_encoded_bytes(), b.name.as_encoded_bytes(), false)
				});
				CmpProxy::show(ShowOpt { cache, cache_name: parent, word: word.into(), ticket });
			}

			Ok::<_, anyhow::Error>(())
		});

		succ!(render!(mem::replace(&mut cmp.visible, false)));
	}
}

impl Trigger {
	fn split_path(s: &str) -> Option<(PathBuf, String)> {
		if s == "~" {
			return None; // We don't autocomplete a `~`, but `~/`
		}

		#[cfg(windows)]
		const SEP: [char; 2] = ['/', '\\'];
		#[cfg(not(windows))]
		const SEP: char = std::path::MAIN_SEPARATOR;

		Some(match s.rsplit_once(SEP) {
			Some(("", c)) => (PathBuf::from(MAIN_SEPARATOR_STR), c.to_owned()),
			Some((p, c)) => (expand_path(p), c.to_owned()),
			None => (CWD.load().to_path_buf(), s.to_owned()),
		})
	}
}

#[cfg(test)]
mod tests {
	use std::path::Path;

	use super::*;

	fn compare(s: &str, parent: &str, child: &str) {
		let (p, c) = Trigger::split_path(s).unwrap();
		let p = p.strip_prefix(yazi_fs::CWD.load().as_ref()).unwrap_or(&p);
		assert_eq!((p, c.as_str()), (Path::new(parent), child));
	}

	#[cfg(unix)]
	#[test]
	fn test_split() {
		yazi_fs::init();
		compare("", "", "");
		compare(" ", "", " ");
		compare("/", "/", "");
		compare("//", "//", "");
		compare("/foo", "/", "foo");
		compare("/foo/", "/foo/", "");
		compare("/foo/bar", "/foo/", "bar");
	}

	#[cfg(windows)]
	#[test]
	fn test_split() {
		yazi_fs::init();
		compare("foo", "", "foo");
		compare(r"foo\", r"foo\", "");
		compare(r"foo\bar", r"foo\", "bar");
		compare(r"foo\bar\", r"foo\bar\", "");
		compare(r"C:\", r"C:\", "");
		compare(r"C:\foo", r"C:\", "foo");
		compare(r"C:\foo\", r"C:\foo\", "");
		compare(r"C:\foo\bar", r"C:\foo\", "bar");
	}
}
