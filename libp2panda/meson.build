if not get_option('libp2panda')
    subdir_done()
endif

libp2panda_deps = [
    dependency('gio-2.0', version: gio_req),
]

pkg = import('pkgconfig')
gnome = import('gnome')
manifest = meson.project_source_root() / 'Cargo.toml'

cargo_options = [
    '--target-dir', cargo_target_dir,
    '--profile', cargo_profile,
    '--manifest-path', manifest,
]

cargo_env_module = cargo_env

packages = []

if get_option('libp2panda')
    packages = [
        {
            'suffix': '',
            'namespace_suffix': '',
            'symbol_prefix_suffix': '',
            'extra_deps': [],
        },
    ]
endif

foreach package : packages
    suffix = package['suffix']
    namespace_suffix = package['namespace_suffix']
    symbol_prefix_suffix = package['symbol_prefix_suffix']

    deps = libp2panda_deps + package['extra_deps']
    link_with = []
    extra_girs = []
    extra_gir_deps = []

    cargo_output = cargo_target_dir / rust_target / f'libp2panda@suffix@.so'.replace('-', '_')

    # Build library with Rust
    libp2panda = custom_target(
        f'libp2panda@suffix@-cargo',
        build_always_stale: true,
        build_by_default: true,
        depends: link_with,
        output: f'Meson-does-not-support-sub-dir-output@suffix@',
        console: true,
        env: cargo_env_module,
        command: [cargo_bin, 'build', cargo_options, '-p', f'libp2panda@suffix@'],
    )

    # Make the build result available to meson. We cannot use the "output"
    # key in the previous target for this since meson prohibits output in subdirs
    libp2panda_copy_library_for_deps = custom_target(
        f'libp2panda@suffix@-copy-library2',
        depends: libp2panda,
        build_by_default: true,
        build_always_stale: true,
        output: f'tmp@suffix@.so',
        command: [
            'cp',
            '-a', cargo_output,
            meson.current_build_dir() / f'tmp@suffix@.so',
        ],
    )

    # We need a shim "lib" object to pass to "generate_gir" later
    libp2panda_shim = library(
        f'p2panda@suffix@-0',
        f'include/p2panda@suffix@.h',
        soversion: '0',
        link_with: link_with,
        include_directories: 'include/',
        dependencies: deps,
        # Mark dirty for "generate_gir" if the header or the rust generated .so changes
        link_depends: [
            # For unknown reasons the header triggers repeated gir regenerates until something else changes
            f'include/p2panda@suffix@.h',
            libp2panda_copy_library_for_deps,
        ],
        install: true,
    )

    x = libp2panda_shim

    # Copy the .so file to its correct location such that "generate_gir" can use it.
    # We have to do this after the shim library target since it generates
    # a useless/empty library.
    libp2panda_copy_library = custom_target(
        f'libp2panda@suffix@-copy-library',
        depends: libp2panda_shim,
        build_always_stale: true,
        build_by_default: true,
        output: f'would-overlap-with-library()@suffix@',
        command: [
            'cp',
            '-a', cargo_output,
            libp2panda_shim.full_path(),
        ],
    )

    install_headers(f'include/p2panda@suffix@.h', subdir: f'p2panda@suffix@-0')

    # Generate a dependency such that the following targets only happen
    # after the .so file has been copied to the correct place
    libp2panda_copy_library_as_dep = declare_dependency(
        sources: libp2panda_copy_library,
    )

    version = run_command(crate_version_bin, f'libp2panda@suffix@', 'gnome', check: true).stdout()

    # Generate .pc file
    pkg.generate(
        name: f'p2panda@suffix@-0',
        description: 'P2panda: GObject bindings for p2panda',
        libraries: deps
        + libp2panda_shim
        + libp2panda_copy_library_as_dep,
        subdirs: f'p2panda@suffix@-0',
        version: version,
    )

    if introspection
        # Generate .gir and .typelib
        libp2panda_gir = gnome.generate_gir(
            libp2panda_shim,
            dependencies: extra_gir_deps + libp2panda_copy_library_as_dep,
            sources: [f'include/p2panda@suffix@.h', libp2panda_copy_library],
            nsversion: '0',
            namespace: f'P2panda@namespace_suffix@',
            symbol_prefix: f'p2panda@symbol_prefix_suffix@',
            includes: ['GLib-2.0', 'GObject-2.0', 'Gio-2.0'] + extra_girs,
            include_directories: meson.current_build_dir(),
            link_with: link_with,
            # Listed as "C headers" in docs
            header: f'p2panda@suffix@.h',
            # Listed as "pkg-config files" in docs
            export_packages: f'p2panda@suffix@-0',
            install: true,
        )
    endif

    if get_option('vapi')
        # Generate .vapi (Vala)
        gnome.generate_vapi(
            f'p2panda@suffix@-0',
            sources: libp2panda_gir[0],
            packages: ['glib-2.0', 'gobject-2.0', 'gio-2.0'],
            gir_dirs: meson.current_build_dir(),
            install: true,
        )
    endif

    if get_option('capi_docs')
        gidocgen = find_program('gi-docgen')

        # Generate HTML documentation
        custom_target(
            f'libp2panda@suffix@-doc',
            # Generate from .gir file
            depends: libp2panda_gir,
            input: [f'doc/libp2panda@suffix@.toml'] + libp2panda_gir[0],
            output: f'libp2panda@suffix@-0',
            command: [
                gidocgen,
                'generate',
                '--config=@INPUT0@',
                '--output-dir=@OUTPUT@',
                '--quiet',
                '--no-namespace-dir',
                '--add-include-path', meson.current_build_dir(),
                '--content-dir', meson.current_source_dir() / 'doc',
                '@INPUT1@',
            ],
            build_by_default: true,
            install: true,
            install_dir: datadir / 'doc',
        )
    endif

endforeach

if get_option('python_tests')
    test('libp2panda-python3', find_program('../tests/libp2panda.py'))
endif
