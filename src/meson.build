pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

gnome.compile_resources('zoom_ms_utility',
  'zoom_ms_utility.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'zoom_ms_utility')
conf.set_quoted('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('APPLICATION_ID', 'io.github.edcorp844.zoommsutility')

configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)

# Copy generated config.rs into source tree
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

cargo_bin = find_program('cargo')

if get_option('buildtype') == 'release'
  cargo_release_flag = '--release'
  rust_target = 'release'
else
  cargo_release_flag = ''
  rust_target = 'debug'
endif

# Define explicit build paths outside meson's src output directory
cargo_home = meson.project_build_root() / 'cargo-home'
target_dir = meson.project_build_root() / 'cargo-target'
manifest_path = meson.project_source_root() / 'Cargo.toml'
binary_source = target_dir / rust_target / meson.project_name()

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'sh', '-c',
    'CARGO_HOME="@0@" "@1@" build @2@ --manifest-path "@3@" --target-dir "@4@" && cp "@5@" "@OUTPUT@"'.format(
      cargo_home,
      cargo_bin.full_path(),
      cargo_release_flag,
      manifest_path,
      target_dir,
      binary_source
    )
  ]
)