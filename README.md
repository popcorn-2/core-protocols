# Core Popcorn2 protocols

This repository contains the [`.pip` source files](https://popcorn-2.github.io/book/ipc.html) for all core Popcorn2 protocols (common name beginning with `core.`, and GUID of the form `00000000-00000000-0000-XXXX`).
With a couple of exceptions (see below), none of these are technically required for a functional Popcorn2 installation.
However, all system libraries, and therefore all compiled exectuables, will assume existence of all of them, so replacing them would require a large amount of work.
In a typical installation these protocols will be compiled to `.pipb` files and installed to `/System/proto/`.

The `core.socket.ctl` protocol (`00000000-00000000-0000-0000`) is so fundamental to the function of Popcorn2 that the system cannot run without it, and it is directly compiled into the kernel.
Additionally, some of the methods are impossible to express in `pip` source files.
An approximation of the definition of `core.socket.ctl` is below (note: this is invalid syntax and will not compile).

```
interface core.socket.ctl(00000000-00000000-0000-0000) {
	static@0 open(endpoint: str, protocols: [uid]) -> impl<>!err;
	func@1 close();
	func@2 extend(protocols: [uid]) -> !err;
}

```
