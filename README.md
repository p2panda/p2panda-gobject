p2panda-gobject - Introspectable GLib/GObject Bindings for p2panda
=============================================================

p2panda-gobject provides a GLib/GObject introspectable API for [p2panda](https://p2panda.org). The library is called `libp2panda`. This project is still in **alpha** and doesn't give any API stability guarantee. This project used [GObject Introspection](https://gi.readthedocs.io/en/latest) to provide support for multiple langugages.

## Getting started

`p2panda-gobject` uses Meson. To install Meson on your system,
follow the [Getting Meson instructions](https://mesonbuild.com/Getting-meson.html). 

### Build libp2panda
To build the project the following commands can be used:
```
meson setup _build --prefix=/usr
meson compile -C _build
```

### Install p2panda-gobject
To install `libp2panda` the following commands can be used:
```
meson install -C _build
```

### Build docs for libp2panda
To generate the docs the following commands can be used:
```
meson setup  -Dcapi_docs=true  --reconfigure
meson compile -C _build
```

### Use libp2panda in python
The tests contain a example on how to use libp2padna in python.
