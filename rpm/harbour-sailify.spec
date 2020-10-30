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
#cargo build -j1 --release --manifest-path %{_sourcedir}/../Cargo.toml

# debug
export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now -Ccodegen-units=1"
cargo build -j1 --manifest-path %{_sourcedir}/../Cargo.toml

# - INSTALL --------------------------------------------------------------------
%install

rm -rf %{buildroot}
install -d %{buildroot}%{_datadir}/applications
install -d %{buildroot}%{_datadir}/%{name}
install -d %{buildroot}%{_datadir}/icons/hicolor/86x86/apps
install -d %{buildroot}%{_bindir}

install -m 755 target/debug/harbour-sailify %{buildroot}%{_bindir}/harbour-sailify

install harbour-sailify.png %{buildroot}%{_datadir}/icons/hicolor/86x86/apps
install harbour-sailify.desktop %{buildroot}%{_datadir}/applications
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


