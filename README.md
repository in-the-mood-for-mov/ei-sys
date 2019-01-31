# ei-sys

This library contains low level bindings for `ei`, a library to handle the Erlang external term format and to communicate with distributed Erlang nodes.

## Compiling This Crate

Because this crate needs the Erlang shell to configure itself and because it statically links to `libei.a`, Erlang needs to be installed where is crate is compiled. Erlang is not needed at runtime.

## Documentation

The official Erlang documentation on [`ei`](http://erlang.org/doc/man/ei.html) and [`ei_connect`](http://erlang.org/doc/man/ei_connect.html) contains information on how to use this library, as well as a complete listing of the types and functions. The [documentation for this crate](https://animalsiknow.github.io/doc/ei_sys/) has additional information that can be of interest to those building on top of this crate, such as the precondition to respect safely whan calling into this crate.
