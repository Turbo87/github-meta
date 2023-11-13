github-meta
==============================================================================

<https://api.github.com/meta> as const structs


Description
------------------------------------------------------------------------------

If the data from <https://api.github.com/meta> is used in a Rust program, it
can be slow to fetch it every time. This crate provides the data as const
structs, so it can be compiled into the program.

An hourly GitHub Actions workflow updates the data in this crate automatically
and releases a new version of the crate if the data has changed.


Usage
------------------------------------------------------------------------------

```rust
fn main() {
    let meta = github_meta::META;
    println!("{meta:#?}");
    let secret_scanning = github_meta::SECRET_SCANNING;
    println!("{secret_scanning:#?}");
}
```


License
------------------------------------------------------------------------------

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
