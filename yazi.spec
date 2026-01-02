%define name yazi
%define version 25.12.29
%define release 1%{?dist}

Summary:  Ping, but with a graph
Name:     %{name}
Version:  %{version}
Release:  %{release}
License:  MIT License
URL:      https://github.com/sxyazi/yazi
Source0:  https://github.com/sxyazi/yazi/archive/refs/tags/v%{version}.tar.gz

%define debug_package %{nil}

BuildRequires: curl
BuildRequires: gcc
BuildRequires: make
BuildRequires: gzip
BuildRequires: upx

%description
Ping, but with a graph.

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
