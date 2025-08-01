use std::{collections::HashMap, ops::Deref};

use indexmap::IndexMap;
use yazi_fs::FilesOp;
use yazi_shared::{timestamp_us, url::{CovUrl, Url}};

#[derive(Default)]
pub struct Selected {
	inner:   IndexMap<CovUrl, u64>,
	parents: HashMap<CovUrl, usize>,
}

impl Selected {
	#[inline]
	pub fn len(&self) -> usize { self.inner.len() }

	#[inline]
	pub fn is_empty(&self) -> bool { self.inner.is_empty() }

	#[inline]
	pub fn values(&self) -> impl Iterator<Item = &Url> { self.inner.keys().map(Deref::deref) }

	#[inline]
	pub fn contains(&self, url: impl AsRef<Url>) -> bool {
		self.inner.contains_key(CovUrl::new(&url))
	}

	#[inline]
	pub fn add(&mut self, url: &Url) -> bool { self.add_same(&[url]) == 1 }

	pub fn add_many(&mut self, urls: &[impl AsRef<Url>]) -> usize {
		let mut grouped: HashMap<_, Vec<_>> = Default::default();
		for u in urls {
			if let Some(p) = u.as_ref().parent_url() {
				grouped.entry(p).or_default().push(u);
			}
		}
		grouped.into_values().map(|v| self.add_same(&v)).sum()
	}

	fn add_same(&mut self, urls: &[impl AsRef<Url>]) -> usize {
		// If it has appeared as a parent
		let urls: Vec<_> =
			urls.iter().map(CovUrl::new).filter(|&u| !self.parents.contains_key(u)).collect();
		if urls.is_empty() {
			return 0;
		}

		// If it has appeared as a child
		let mut parent = urls[0].parent_url();
		let mut parents = vec![];
		while let Some(u) = parent {
			if self.inner.contains_key(&u) {
				return 0;
			}

			parent = u.parent_url();
			parents.push(u);
		}

		let (now, len) = (timestamp_us(), self.inner.len());
		self.inner.extend(urls.iter().enumerate().map(|(i, &u)| (u.clone(), now + i as u64)));

		for u in parents {
			*self.parents.entry(u).or_insert(0) += self.inner.len() - len;
		}
		urls.len()
	}

	#[inline]
	pub fn remove(&mut self, url: &Url) -> bool { self.remove_same(&[url]) == 1 }

	pub fn remove_many(&mut self, urls: &[impl AsRef<Url>]) -> usize {
		let mut grouped: HashMap<_, Vec<_>> = Default::default();
		for u in urls {
			if let Some(p) = u.as_ref().parent_url() {
				grouped.entry(p).or_default().push(u);
			}
		}

		let affected = grouped.into_values().map(|v| self.remove_same(&v)).sum();
		if affected > 0 {
			self.inner.sort_unstable_by(|_, a, _, b| a.cmp(b));
		}

		affected
	}

	fn remove_same(&mut self, urls: &[impl AsRef<Url>]) -> usize {
		let count = urls.iter().filter_map(|u| self.inner.swap_remove(CovUrl::new(u))).count();
		if count == 0 {
			return 0;
		}

		let mut parent = CovUrl::new(&urls[0]).parent_url();
		while let Some(u) = parent {
			let n = self.parents.get_mut(&u).unwrap();

			*n -= count;
			if *n == 0 {
				self.parents.remove(&u);
			}

			parent = u.parent_url();
		}
		count
	}

	pub fn clear(&mut self) {
		self.inner.clear();
		self.parents.clear();
	}

	pub fn apply_op(&mut self, op: &FilesOp) {
		let (removal, addition) = op.diff_recoverable(|u| self.contains(u));
		if !removal.is_empty() {
			self.remove_many(&removal);
		}
		if !addition.is_empty() {
			self.add_many(&addition);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn url(s: &str) -> Url { Url::try_from(s).unwrap() }

	#[test]
	fn test_insert_non_conflicting() {
		let mut s = Selected::default();

		assert!(s.add(&url("/a/b")));
		assert!(s.add(&url("/c/d")));
		assert_eq!(s.inner.len(), 2);
	}

	#[test]
	fn test_insert_conflicting_parent() {
		let mut s = Selected::default();

		assert!(s.add(&url("/a")));
		assert!(!s.add(&url("/a/b")));
	}

	#[test]
	fn test_insert_conflicting_child() {
		let mut s = Selected::default();

		assert!(s.add(&url("/a/b/c")));
		assert!(!s.add(&url("/a/b")));
		assert!(s.add(&url("/a/b/d")));
	}

	#[test]
	fn test_remove() {
		let mut s = Selected::default();

		assert!(s.add(&url("/a/b")));
		assert!(!s.remove(&url("/a/c")));
		assert!(s.remove(&url("/a/b")));
		assert!(!s.remove(&url("/a/b")));
		assert!(s.inner.is_empty());
		assert!(s.parents.is_empty());
	}

	#[test]
	fn insert_many_success() {
		let mut s = Selected::default();

		assert_eq!(
			3,
			s.add_same(&[&url("/parent/child1"), &url("/parent/child2"), &url("/parent/child3")])
		);
	}

	#[test]
	fn insert_many_with_existing_parent_fails() {
		let mut s = Selected::default();

		s.add(&url("/parent"));
		assert_eq!(0, s.add_same(&[&url("/parent/child1"), &url("/parent/child2")]));
	}

	#[test]
	fn insert_many_with_existing_child_fails() {
		let mut s = Selected::default();

		s.add(&url("/parent/child1"));
		assert_eq!(2, s.add_same(&[&url("/parent/child1"), &url("/parent/child2")]));
	}

	#[test]
	fn insert_many_empty_urls_list() {
		let mut s = Selected::default();

		assert_eq!(0, s.add_same(&[] as &[&Url]));
	}

	#[test]
	fn insert_many_with_parent_as_child_of_another_url() {
		let mut s = Selected::default();

		s.add(&url("/parent/child"));
		assert_eq!(0, s.add_same(&[&url("/parent/child/child1"), &url("/parent/child/child2")]));
	}
	#[test]
	fn insert_many_with_direct_parent_fails() {
		let mut s = Selected::default();

		s.add(&url("/a"));
		assert_eq!(0, s.add_same(&[&url("/a/b")]));
	}

	#[test]
	fn insert_many_with_nested_child_fails() {
		let mut s = Selected::default();

		s.add(&url("/a/b"));
		assert_eq!(0, s.add_same(&[&url("/a")]));
		assert_eq!(1, s.add_same(&[&url("/b"), &url("/a")]));
	}

	#[test]
	fn insert_many_sibling_directories_success() {
		let mut s = Selected::default();

		assert_eq!(2, s.add_same(&[&url("/a/b"), &url("/a/c")]));
	}

	#[test]
	fn insert_many_with_grandchild_fails() {
		let mut s = Selected::default();

		s.add(&url("/a/b"));
		assert_eq!(0, s.add_same(&[&url("/a/b/c")]));
	}

	#[test]
	fn test_insert_many_with_remove() {
		let mut s = Selected::default();

		let child1 = url("/parent/child1");
		let child2 = url("/parent/child2");
		let child3 = url("/parent/child3");
		assert_eq!(3, s.add_same(&[&child1, &child2, &child3]));

		assert!(s.remove(&child1));
		assert_eq!(s.inner.len(), 2);
		assert!(!s.parents.is_empty());

		assert!(s.remove(&child2));
		assert!(!s.parents.is_empty());

		assert!(s.remove(&child3));
		assert!(s.inner.is_empty());
		assert!(s.parents.is_empty());
	}
}
