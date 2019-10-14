# libstrophe-sys-bindgen

## Documentation

See [full documentation](https://docs.rs/libstrophe-sys-bindgen)

## Usage

Add this to your Cargo.toml:
```
[dependencies]
libstrophe-sys-bindgen = "2.1.0"
```

[![Build Status](https://travis-ci.org/twistedfall/libstrophe.svg?branch=master)](https://travis-ci.org/twistedfall/libstrophe)

## libstrophe C library bindings

This crate provides bindings to [libstrophe] C library which enables you the creation of XMPP
clients and servers. The bindings were statically generated using [bindgen] so the crate doesn't
have a hard dependency on bindgen. If you still want to regenerate the bindings during building
of the create, enable `buildtime_bindgen` feature.

Usage of this crate creates runtime dependency on libstrophe.so so be sure to install that using
your package manager.

Current bindings were generated from libstrophe version: 0.9.3

The difference from [libstrophe-sys] crate is the this one is automatically generated hence
easier to maintain.

This crate contains only C bindings, for Rust ergonomic interface see [libstrophe][libstrophe_crate] crate.

[libstrophe]: http://strophe.im/libstrophe
[bindgen]: https://crates.io/crates/bindgen
[libstrophe-sys]: https://crates.io/crates/libstrophe-sys
[libstrophe_crate]: https://crates.io/crates/libstrophe

License: LGPL-3.0
