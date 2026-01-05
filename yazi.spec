Name:           yazi
Version:        26.1.4
Release:        1%{?dist}
Summary:        Blazing fast terminal file manager written in Rust, based on async I/O

License:        MIT
URL:            https://github.com/sxyazi/yazi
Source0:        https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

%define debug_package %{nil}

# --- Amazon Linux 2023 specific fixes ---
# RHEL 9/10 do NOT need these and must not be affected
%if 0%{?amzn}
%global _package_note_file %{nil}
%global _package_note_flags %{nil}
%global rustflags %{nil}
%endif

BuildRequires:  gcc
BuildRequires:  make
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconf-pkg-config
BuildRequires:  gzip

%description
Yazi (means "duck") is a terminal file manager written in Rust, based on
non-blocking async I/O. It provides a fast, user-friendly, and customizable
terminal file management experience.

%prep
%autosetup -n yazi-%{version}

%build
# AL2023 may inject broken RUSTFLAGS; harmless on RHEL
unset RUSTFLAGS || true

# Explicit flags, safe on all platforms
export RUSTFLAGS="-Copt-level=3 -Cdebuginfo=2 --cap-lints=warn"

# Networked build (same behaviour as your RHEL spec)
cargo build --release --locked

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
