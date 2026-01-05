%define name yazi
%define version 26.1.4
%define release 1%{?dist}

Summary:  Blazing fast terminal file manager written in Rust, based on async I/O
Name:     %{name}
Version:  %{version}
Release:  %{release}
License:  MIT License
URL:      https://github.com/sxyazi/yazi
Source0:  https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

%global _package_note_flags %{nil}
%define debug_package %{nil}
%global _package_note_file %{nil}
%undefine _package_note_file
%undefine _package_note_flags

BuildRequires: curl
BuildRequires: gcc
BuildRequires: make
BuildRequires: gzip
BuildRequires: upx

%description
Yazi (means "duck") is a terminal file manager written in Rust, based on non-blocking async I/O. 
It aims to provide an efficient, user-friendly, and customizable file management experience.

%prep
%setup -q -n yazi-%{version}

%build
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$PATH:$HOME/.cargo/bin"
cargo build --release --locked
strip --strip-all target/release/%{name}
mkdir -p %{buildroot}/%{_bindir}

%install
mkdir -p %{buildroot}/%{_bindir}/

install -m 755 target/release/yazi %{buildroot}/%{_bindir}/
install -m 755 target/release/ya %{buildroot}/%{_bindir}/

%files
%license LICENSE
%doc README.md
%{_bindir}/ya
%{_bindir}/yazi
