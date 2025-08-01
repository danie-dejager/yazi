use std::collections::HashSet;

use tracing::debug;
use yazi_shared::url::{CovUrl, Url};

use super::Tasks;

impl Tasks {
	pub fn file_cut(&self, src: &[&CovUrl], dest: &Url, force: bool) {
		for &u in src {
			let to = dest.join(u.file_name().unwrap());
			if force && *u == to {
				debug!("file_cut: same file, skipping {:?}", to);
			} else {
				self.scheduler.file_cut(u.0.clone(), to, force);
			}
		}
	}

	pub fn file_copy(&self, src: &[&CovUrl], dest: &Url, force: bool, follow: bool) {
		for &u in src {
			let to = dest.join(u.file_name().unwrap());
			if force && *u == to {
				debug!("file_copy: same file, skipping {:?}", to);
			} else {
				self.scheduler.file_copy(u.0.clone(), to, force, follow);
			}
		}
	}

	pub fn file_link(&self, src: &HashSet<CovUrl>, dest: &Url, relative: bool, force: bool) {
		for u in src {
			let to = dest.join(u.file_name().unwrap());
			if force && *u == to {
				debug!("file_link: same file, skipping {:?}", to);
			} else {
				self.scheduler.file_link(u.0.clone(), to, relative, force);
			}
		}
	}

	pub fn file_hardlink(&self, src: &HashSet<CovUrl>, dest: &Url, force: bool, follow: bool) {
		for u in src {
			let to = dest.join(u.file_name().unwrap());
			if force && *u == to {
				debug!("file_hardlink: same file, skipping {:?}", to);
			} else {
				self.scheduler.file_hardlink(u.0.clone(), to, force, follow);
			}
		}
	}

	pub fn file_remove(&self, targets: Vec<Url>, permanently: bool) {
		for u in targets {
			if permanently {
				self.scheduler.file_delete(u);
			} else {
				self.scheduler.file_trash(u);
			}
		}
	}
}
