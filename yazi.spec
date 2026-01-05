Name:           yazi
Version:        26.1.4
Release:        1%{?dist}
Summary:        Blazing fast terminal file manager written in Rust, based on async I/O

License:        MIT
URL:            https://github.com/sxyazi/yazi
Source0:        https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

# --- Disable problematic RPM/Rust macros (AL2023 safe) ---
%define debug_package %{nil}
%global _package_note_file %{nil}
%global _package_note_flags %{nil}
%global rustflags %{nil}

BuildRequires:  gcc
BuildRequires:  make
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig
BuildRequires:  gzip

%description
Yazi (means "duck") is a terminal file manager written in Rust, based on
non-blocking async I/O. It provides a fast, user-friendly, and customizable
terminal file management experience.

%prep
%autosetup -n yazi-%{version}

%build
# Explicit, minimal, portable Rust flags
export RUSTFLAGS="-Copt-level=3 -Cdebuginfo=2 --cap-lints=warn"

cargo build \
    --release \
    --locked

%install
install -Dpm0755 target/release/yazi %{buildroot}%{_bindir}/yazi
install -Dpm0755 target/release/ya   %{buildroot}%{_bindir}/ya

%files
%license LICENSE
%doc README.md
%{_bindir}/yazi
%{_bindir}/ya
