use std::io;

use yazi_shared::url::Url;

use crate::services::Local;

#[inline]
pub async fn canonicalize(url: impl AsRef<Url>) -> io::Result<Url> {
	if let Some(path) = url.as_ref().as_path() {
		Local::canonicalize(path).await.map(Into::into)
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn create(url: impl AsRef<Url>) -> io::Result<tokio::fs::File> {
	if let Some(path) = url.as_ref().as_path() {
		Local::create(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn create_dir(url: impl AsRef<Url>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::create_dir(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn create_dir_all(url: impl AsRef<Url>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::create_dir_all(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn hard_link(original: impl AsRef<Url>, link: impl AsRef<Url>) -> io::Result<()> {
	if let (Some(original), Some(link)) = (original.as_ref().as_path(), link.as_ref().as_path()) {
		Local::hard_link(original, link).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn metadata(url: impl AsRef<Url>) -> io::Result<std::fs::Metadata> {
	if let Some(path) = url.as_ref().as_path() {
		Local::metadata(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn open(url: impl AsRef<Url>) -> io::Result<tokio::fs::File> {
	if let Some(path) = url.as_ref().as_path() {
		Local::open(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn read_dir(url: impl AsRef<Url>) -> io::Result<tokio::fs::ReadDir> {
	if let Some(path) = url.as_ref().as_path() {
		Local::read_dir(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub fn read_dir_sync(url: impl AsRef<Url>) -> io::Result<std::fs::ReadDir> {
	if let Some(path) = url.as_ref().as_path() {
		Local::read_dir_sync(path)
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn read_link(url: impl AsRef<Url>) -> io::Result<Url> {
	if let Some(path) = url.as_ref().as_path() {
		Local::read_link(path).await.map(Into::into)
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn remove_dir(url: impl AsRef<Url>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::remove_dir(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn remove_dir_all(url: impl AsRef<Url>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::remove_dir_all(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn remove_file(url: impl AsRef<Url>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::remove_file(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn rename(from: impl AsRef<Url>, to: impl AsRef<Url>) -> io::Result<()> {
	if let (Some(from), Some(to)) = (from.as_ref().as_path(), to.as_ref().as_path()) {
		Local::rename(from, to).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn symlink_dir(original: impl AsRef<Url>, link: impl AsRef<Url>) -> io::Result<()> {
	if let (Some(original), Some(link)) = (original.as_ref().as_path(), link.as_ref().as_path()) {
		Local::symlink_dir(original, link).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn symlink_file(original: impl AsRef<Url>, link: impl AsRef<Url>) -> io::Result<()> {
	if let (Some(original), Some(link)) = (original.as_ref().as_path(), link.as_ref().as_path()) {
		Local::symlink_file(original, link).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn symlink_metadata(url: impl AsRef<Url>) -> io::Result<std::fs::Metadata> {
	if let Some(path) = url.as_ref().as_path() {
		Local::symlink_metadata(path).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

pub fn symlink_metadata_sync(url: impl AsRef<Url>) -> io::Result<std::fs::Metadata> {
	if let Some(path) = url.as_ref().as_path() {
		Local::symlink_metadata_sync(path)
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}

#[inline]
pub async fn write(url: impl AsRef<Url>, contents: impl AsRef<[u8]>) -> io::Result<()> {
	if let Some(path) = url.as_ref().as_path() {
		Local::write(path, contents).await
	} else {
		Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported filesystem"))
	}
}
