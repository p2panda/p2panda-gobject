project(
  'p2panda',
  'rust',
  'c',
  version: '0.1',
  meson_version: '>=1.2',
)

gio_req = '>=2.60'

compat_version = '2+'

datadir = get_option('prefix') / get_option('datadir')
bindir = get_option('prefix') / get_option('bindir')
libdir = get_option('prefix') / get_option('libdir')
includedir = get_option('prefix') / get_option('includedir')

cargo_target_dir = meson.project_build_root() / 'cargo-target'
cargo_home = meson.project_build_root() / 'cargo-home'
cargo_manifest = meson.project_source_root() / 'Cargo.toml'

cargo_bin = find_program('cargo')
crate_version_bin = find_program(meson.project_source_root() / 'build-aux/crates-version.py')

cargo_env = environment()
cargo_env.set('CARGO_HOME', cargo_home)

if get_option('profile') == 'release'
  rust_target = 'release'
  cargo_profile = 'release'

  if get_option('lto') == 'auto'
    cargo_version_details = run_command(cargo_bin, 'version', '-v', check: true).stdout().strip()

    # Disable LTO for architectures other than x86_64 and aarch64
    # There have been frequent built issues with memory limits on 32-bit architectures
    if not cargo_version_details.contains('host: x86_64-') and not cargo_version_details.contains('host: aarch64-')
      message('LTO automatically disabled')
      cargo_env.set('CARGO_PROFILE_RELEASE_LTO', 'false')
    endif
  elif get_option('lto') == 'false'
    cargo_env.set('CARGO_PROFILE_RELEASE_LTO', 'false')
  endif

else
  rust_target = 'debug'
  cargo_profile = 'dev'
endif

introspection = (get_option('introspection') or get_option('vapi') or get_option('capi_docs'))

if get_option('introspection') != introspection
  summary('introspection', introspection, section: 'Overwritten options')
endif

subdir('libp2panda')
