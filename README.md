# p2panda-gobject

`p2panda-gobject` provides introspectable GLib/GObject bindings for [p2panda's Node
API](https://p2panda.org). The library is called `libp2panda`.

This project used [GObject Introspection](https://gi.readthedocs.io/en/latest) to provide support
for multiple langugages.

Next to this repository you'll find non-GObject bindings (Go, Node.js, Python) using UniFFI for
p2panda in [`p2panda-ffi`](https://github.com/p2panda/p2panda-ffi).

> 🚧 This library is under active development and the APIs are not yet considered stable for
> production use. Core data types and user-facing APIs may still undergo breaking changes. Stability
> guarantees will improve with the release of v1.0.0.

## Getting started

`p2panda-gobject` uses Meson. To install Meson on your system, follow the [Getting Meson
instructions](https://mesonbuild.com/Getting-meson.html). 

### Build libp2panda

To build the project the following commands can be used:

```bash
meson setup _build --prefix=/usr
meson compile -C _build
```

### Install p2panda-gobject

To install `libp2panda` the following commands can be used:

```bash
meson install -C _build
```

### Build docs for libp2panda

To generate the docs the following commands can be used:

```bash
meson setup  -Dcapi_docs=true  --reconfigure
meson compile -C _build
```

### Use libp2panda in python

The tests contain an example on how to use libp2panda in python.

## Credits

Thanks a lot to Sophie Herold, for writing [libglycin](https://gitlab.gnome.org/GNOME/glycin) that
was used as a base for figuring out how to write the bindings and Sergey Bugaev for reviewing the
GLib introspection API.
