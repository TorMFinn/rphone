Old Phone Project
=================

Cross Compiling
===============

In order to cross compile you must add the rust compile target.

rustup target add armv7-unknown-linux-gnueabi

Make sure you have a gcc cross compiler installed as well.

Then in ~/.cargo/config add which linker to use for the given target.

[target:wq
