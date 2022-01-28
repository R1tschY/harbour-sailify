Name:       harbour-sailify
Summary:    User-friendly Spotify client for Sailfish OS
Version:    0.1
Release:    0
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
BuildRequires:  pkgconfig(libpulse-simple)
BuildRequires:  pkgconfig(openssl)
BuildRequires:  desktop-file-utils
BuildRequires:  cmake
BuildRequires:  ninja
BuildRequires:  rust >= 1.48
BuildRequires:  rust-std-static >= 1.48
BuildRequires:  cargo
BuildRequires:  cbindgen


%description
A Spotify client for Sailfish OS focused on usability and stability.

%define BUILD_DIR "$PWD"/target

# - PREP -----------------------------------------------------------------------
%prep
%setup -q -n %{name}-%{version}

# - BUILD ----------------------------------------------------------------------
%build
mkdir -p "%{BUILD_DIR}"

#
# App enviroment

export SAILIFY_PACKAGE_VERSION="%{version}-%{release}"
export $(egrep -v '^#' %{_sourcedir}/../.env | xargs)

#
# Rust cross-compile environment
# See https://github.com/sailfishos/gecko-dev/blob/master/rpm/xulrunner-qt5.spec

case "$DEB_BUILD_ARCH_CPU" in
    armv7hl)
        export SB2_TARGET=armv7-unknown-linux-gnueabihf
        ;;

    aarch64)
        export SB2_TARGET=aarch64-unknown-linux-gnu
        ;;

    i486)
        export SB2_TARGET=i686-unknown-linux-gnu
        ;;

    *)
        echo "Unknown arch $DEB_BUILD_ARCH_CPU"
        exit 1
        ;;
esac

export LIBDIR='%{_libdir}'

# When cross-compiling under SB2 rust needs to know what arch to emit
# when nothing is specified on the command line. That usually defaults
# to "whatever rust was built as" but in SB2 rust is accelerated and
# would produce x86 so this is how it knows differently. Not needed
# for native x86 builds
export SB2_RUST_TARGET_TRIPLE=$SB2_TARGET
export RUST_HOST_TARGET=$SB2_TARGET

export RUST_TARGET=$SB2_TARGET
export TARGET=$SB2_TARGET
export HOST=$SB2_TARGET
export SB2_TARGET=$SB2_TARGET

if [ "$DEB_BUILD_ARCH_CPU" == armv7hl ] || [ "$DEB_BUILD_ARCH_CPU" == aarch64 ]; then
    export CROSS_COMPILE=$SB2_TARGET

    # This avoids a malloc hang in sb2 gated calls to execvp/dup2/chdir
    # during fork/exec. It has no effect outside sb2 so doesn't hurt
    # native builds.
    export SB2_RUST_EXECVP_SHIM="/usr/bin/env LD_PRELOAD=/usr/lib/libsb2/libsb2.so.1 /usr/bin/env"
    export SB2_RUST_USE_REAL_EXECVP=Yes
    export SB2_RUST_USE_REAL_FN=Yes
fi

export CC=gcc
export CXX=g++
export AR="gcc-ar"
export NM="gcc-nm"
export RANLIB="gcc-ranlib"
export PKG_CONFIG="pkg-config"

export CARGO_BUILD_TARGET=$SB2_TARGET

#
# Cargo

export CARGO_PROFILE_RELEASE_LTO=fat
export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now -Ccodegen-units=1 %{?rustflags}"
export CARGO_INCREMENTAL=0

if [ "$SAILFISH_SDK_FRONTEND" == "qtcreator" ] ; then
  cargo build --release -j1 --target-dir=$PWD --manifest-path %{_sourcedir}/../Cargo.toml
else
  cargo build --release -j1 --target-dir=%BUILD_DIR --manifest-path %{_sourcedir}/../Cargo.toml
fi

#
# CMake
CMAKE_BUILD_DIR="%{BUILD_DIR}/${SB2_TARGET}/release"
SOURCE_DIR=`readlink -f %{_sourcedir}/..`

# Ninja seems not to work for i686
if [ "$DEB_BUILD_ARCH_CPU" == i686 ]; then
    GENERATOR="Unix Makefiles"
else
    GENERATOR="Ninja"
fi

if [ "$SAILFISH_SDK_FRONTEND" == "qtcreator" ] ; then
  CMAKE_BUILD_TYPE="Debug"
else
  CMAKE_BUILD_TYPE="RelWithDebInfo"
fi

if [ "$SAILFISH_SDK_FRONTEND" == "qtcreator" ] ; then
    cmake \
      -GNinja \
      -DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE \
      -DBUILD_SHARED_LIBS=OFF \
      -DCMAKE_INSTALL_PREFIX=/usr \
      -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
      -DRUST_TARGET_DIR=${SB2_TARGET}/release \
      -DSAILFISHOS=ON
    cmake --build . -- %{?_smp_mflags}
else
    cmake \
      -DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE \
      -DBUILD_SHARED_LIBS=OFF \
      -DCMAKE_INSTALL_PREFIX=/usr \
      -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
      -DSAILFISHOS=ON \
      -G "$GENERATOR" \
      -S "$SOURCE_DIR" \
      -B "$CMAKE_BUILD_DIR"
    cmake --build "$CMAKE_BUILD_DIR" -- %{?_smp_mflags}
fi

# - INSTALL --------------------------------------------------------------------
%install
case "$DEB_BUILD_ARCH_CPU" in
    armv7hl)
        export SB2_TARGET=armv7-unknown-linux-gnueabihf
        ;;

    aarch64)
        export SB2_TARGET=aarch64-unknown-linux-gnu
        ;;

    i486)
        export SB2_TARGET=i686-unknown-linux-gnu
        ;;

    *)
        echo "Unknown arch $DEB_BUILD_ARCH_CPU"
        exit 1
        ;;
esac
CMAKE_BUILD_DIR="%{BUILD_DIR}/${SB2_TARGET}/release"

rm -rf %{buildroot}
DESTDIR=%{buildroot} cmake --build "$CMAKE_BUILD_DIR" --target install

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


