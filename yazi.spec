Name:           yazi
Version:        26.1.4
Release:        1%{?dist}
Summary:        Blazing fast terminal file manager written in Rust, based on async I/O

License:        MIT
URL:            https://github.com/sxyazi/yazi
Source0:        https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

%define debug_package %{nil}

%if 0%{?amzn}
%global __cargo_skip_rpm_macros 1
%endif

BuildRequires:  gcc
BuildRequires:  make
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconf-pkg-config
BuildRequires:  gzip
%if 0%{?amzn}
BuildRequires: rust-packaging
BuildRequires: rust-srpm-macros
%endif

%description
Yazi (means "duck") is a terminal file manager written in Rust, based on
non-blocking async I/O. It provides a fast, user-friendly, and customizable
terminal file management experience.

%prep
%autosetup -n yazi-%{version}

%build
%if 0%{?amzn}
# AL2023: do NOT use RPM Rust macros at all
unset RUSTFLAGS
export RUSTFLAGS="-Copt-level=3 -Cdebuginfo=2 --cap-lints=warn"
cargo build --release --locked
%else
# RHEL 9/10: normal Rust RPM behaviour
cargo build --release --locked
%endif

%install
install -Dpm0755 target/release/yazi %{buildroot}%{_bindir}/yazi
install -Dpm0755 target/release/ya   %{buildroot}%{_bindir}/ya

%files
%license LICENSE
%doc README.md
%{_bindir}/yazi
%{_bindir}/ya

%changelog
* Mon Jan 05 2026 Danie de Jager <danie.dejager@gmail.com> - 26.1.4-1
- Unified spec for RHEL 9/10 and Amazon Linux 2023
