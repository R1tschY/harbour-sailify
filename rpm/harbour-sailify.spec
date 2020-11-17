Name:       harbour-sailify
Summary:    User-friendly Spotify client for Sailfish OS
Version:    1.0
Release:    1
Group:      Qt/Qt
License:    LICENSE
URL:        http://example.org/
Source0:    %{name}-%{version}.tar.bz2
Requires:   sailfishsilica-qt5 >= 0.10.9
BuildRequires:  pkgconfig(sailfishapp) >= 1.0.2
BuildRequires:  pkgconfig(Qt5Core)
BuildRequires:  pkgconfig(Qt5Qml)
BuildRequires:  pkgconfig(Qt5Quick)
BuildRequires:  pkgconfig(libpulse)
BuildRequires:  pkgconfig(openssl)
BuildRequires:  desktop-file-utils
BuildRequires:  rust
BuildRequires:  cargo


%description
A Spotify client for Sailfish OS focused on usability and stability.

# - PREP -----------------------------------------------------------------------
%prep
%setup -q -n %{name}-%{version}

# - BUILD ----------------------------------------------------------------------
%build

export RPM_VERSION=%{version}

# release
#export CARGO_INCREMENTAL=0
#cargo build -j1 --release --target-dir=target --locked --manifest-path %{_sourcedir}/../Cargo.toml

# debug
export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now -Ccodegen-units=1"
cargo build -j1 --target-dir=target --locked --manifest-path %{_sourcedir}/../Cargo.toml

touch Makefile

# - INSTALL --------------------------------------------------------------------
%install

rm -rf %{buildroot}
install -d %{buildroot}%{_datadir}/%{name}

install -Dm 755 target/debug/%{name} -t %{buildroot}%{_bindir}

install -Dm 644 harbour-sailify.png -t %{buildroot}%{_datadir}/icons/hicolor/86x86/apps
install -Dm 644 harbour-sailify.desktop -t %{buildroot}%{_datadir}/applications
cp -r qml %{buildroot}%{_datadir}/%{name}/qml

desktop-file-install --delete-original       \
  --dir %{buildroot}%{_datadir}/applications             \
   %{buildroot}%{_datadir}/applications/*.desktop

# - FILES ----------------------------------------------------------------------
%files

%defattr(-,root,root,-)
%{_bindir}
%{_datadir}/%{name}/qml
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/86x86/apps/%{name}.png


