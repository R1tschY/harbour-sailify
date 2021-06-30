Name:       harbour-sailify
Summary:    User-friendly Spotify client for Sailfish OS
Version:    1.0
Release:    1
Group:      Qt/Qt
License:    LICENSE
URL:        http://example.org/
Source0:    %{name}-%{version}.tar.bz2
Requires:   sailfishsilica-qt5 >= 0.10.9
Requires:   mpris-qt5-qml-plugin
Requires:   qml(org.freedesktop.contextkit)
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

export $(egrep -v '^#' %{_sourcedir}/../.env | xargs)
export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now -Ccodegen-units=1"
export TMPDIR=%{_sourcedir}/../.tmp

mkdir -p "$TMPDIR"

# release
export CARGO_INCREMENTAL=0
cargo build --release --target-dir=target --locked --manifest-path %{_sourcedir}/../Cargo.toml

# debug
#cargo build --target-dir=target --locked --manifest-path %{_sourcedir}/../Cargo.toml

touch Makefile

# - INSTALL --------------------------------------------------------------------
%install

rm -rf %{buildroot}
install -d %{buildroot}%{_datadir}/%{name}

install -Dm 755 target/release/%{name} -t %{buildroot}%{_bindir}

for size in 86 108 128 172
do
  install -Dm 644 %{_sourcedir}/../res/${size}x${size}/harbour-sailify.png -t %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps
done

install -Dm 644 %{_sourcedir}/../harbour-sailify.desktop -t %{buildroot}%{_datadir}/applications
cp -r %{_sourcedir}/../qml %{buildroot}%{_datadir}/%{name}/qml

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
%{_datadir}/icons/hicolor/108x108/apps/%{name}.png
%{_datadir}/icons/hicolor/128x128/apps/%{name}.png
%{_datadir}/icons/hicolor/172x172/apps/%{name}.png


