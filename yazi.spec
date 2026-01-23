Name:           yazi
Version:        26.1.22
Release:        1%{?dist}
Summary:        Blazing fast terminal file manager written in Rust, based on async I/O

License:        MIT
URL:            https://github.com/sxyazi/yazi
Source0:        https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

%define debug_package %{nil}

%if 0%{?amzn}
%global __cargo_skip_rpm_macros 1
%endif
%if 0%{?rhel} == 9 || 0%{?rhel} == 10
%global use_rustup 1
%endif

BuildRequires:  gcc
BuildRequires:  make
BuildRequires:  pkgconf-pkg-config
BuildRequires:  gzip
%if !0%{?use_rustup}
BuildRequires: cargo
BuildRequires: rust
%endif
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
# ---------------- Toolchain selection ----------------

%if 0%{?use_rustup}
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$PATH:$HOME/.cargo/bin"
%endif

%if 0%{?amzn}
unset RUSTFLAGS
export RUSTFLAGS="-Copt-level=3 -Cdebuginfo=2 --cap-lints=warn"
%endif

cargo build --release --locked

strip --strip-all target/release/%{name}
mkdir -p %{buildroot}/%{_bindir}

%install
install -Dpm0755 target/release/yazi %{buildroot}%{_bindir}/yazi
install -Dpm0755 target/release/ya   %{buildroot}%{_bindir}/ya

%files
%license LICENSE
%doc README.md
%{_bindir}/yazi
%{_bindir}/ya

%changelog
* Thu Jan 22 2026 - Danie de Jager <danie.dejager@gmail.com> - 26.1.22-1
* Mon Jan 05 2026 - Danie de Jager <danie.dejager@gmail.com> - 26.1.4-1
- Unified spec for RHEL 9/10 and Amazon Linux 2023
