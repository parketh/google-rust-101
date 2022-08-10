# Rust 101

go/rust-101-codelab

[TOC]

Rust 101 is a g2g session for learning the basics of the Rust programming
language. To see upcoming sessions check out the Grow page at
http://go/rust-101.

For more help with Rust, see http://g/rust-users.

## Prework

Before coming to the class I recommend that you have a working Rust + Cargo
environment and download the codelab files.

### Installing Rust

See go/rust#setup

### Get the codelab files

The codelab files are available at go/rust-101-files. Download the files and
extract them to a local directory on your workstation.

You can also find the codelab files in google3/experimental/kevincox/rust101.

If you choose to run the codelab on a Cloudtop/Ubiquity instance, you can create
a new citc client and work directly in `google3/experimental/kevincox/rust101`.

Otherwise you can copy the codelab files to a local directory:

```sh
rsync -a --chmod=+w  /google/src/head/depot/google3/experimental/kevincox/rust101 ~
```

### Testing

```sh
cd rust101
cargo test solution
```

If all the tests pass then you are good to go. If some of the tests fail I would
appreciate if you send details to rust-instructors@google.com.

## Codelab

The codelab instructions are in the [rust101/src/codelab.rs](http://google3/experimental/kevincox/rust101/src/codelab.rs) file, simply start from the top and fix the code as you go.