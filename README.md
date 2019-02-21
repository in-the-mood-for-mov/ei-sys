# ei-sys

This library contains low level bindings for `ei`, a library to handle the Erlang external term format and to communicate with distributed Erlang nodes.

## Compiling This Crate

The library with which this crate links is not usually included in the default link path. There are two ways to configure this crate.

* If you have a full Erlang installation, the build script will invoke the Erlang shell to find where it installed its libraries. This is the recommended way to configure this crate.
* You can set the `EI_LINK_SEARCH` environment variable to the directory that contains libei.

### Windows

The official Erlang binaries are compiled with MSVC 11 (Visual Studio 2012), which is not ABI compatible with binaries built with more recent MSVC versions, including those that are compatible with Rust. You will need to build you own Erlang. You can follow [the official instructions](https://github.com/erlang/otp/blob/master/HOWTO/INSTALL-WIN32.md), but change the `PATH`, `LIBPATH`, `LIB`, and `INCLUDE` environment for those of the version of MSVC you are using with Rust.
 
For example, for a `x86_64` build with Visual Studio 2017, open the *x64 Native Tool Command Promp for VS 2017*, and then you can see the values of the variables listed above by typing
```
echo %PATH%
echo %LIBPATH%
echo %LIB%
echo %INCLUDE%
```

Note also that Erlang does not build with mingw. Thus you cannot use this crate with a `{i686|x86_64}-pc-windows-gnu` version of Rust, i.e. you need to use `{i686|x86_64}-pc-windows-msvc`.

## Documentation

The official Erlang documentation on [`ei`](http://erlang.org/doc/man/ei.html) and [`ei_connect`](http://erlang.org/doc/man/ei_connect.html) contains information on how to use this library, as well as a complete listing of the types and functions. The [documentation for this crate](https://animalsiknow.github.io/doc/ei_sys/) has additional information that can be of interest to those building on top of this crate, such as the precondition to respect safely whan calling into this crate.
