[![Latest Version]][crates.io] [![Docs]][docs.rs] ![License]

[Latest Version]: https://img.shields.io/crates/v/rubenvy.svg
[crates.io]: https://crates.io/crates/rubenvy
[Docs]: https://img.shields.io/docsrs/rubenvy/0.1.1
[docs.rs]: https://docs.rs/rubenvy/0.1.1/rubenvy/index.html
[License]: https://img.shields.io/crates/l/rubenvy

Support for [Ruby-style](https://github.com/bkeepers/dotenv/blob/c6e583a/README.md#what-other-env-files-can-i-use) dotenv loading priorities utilizing [dotenvy](https://github.com/allan2/dotenvy).

# Example
```rust
// Load environment variables with the following priority:
// 1. `.env.development.local`
// 2. `.env.local`
// 3. `.env.development`
// 4. `.env`
rubenvy::rubenvy(Environment::Development).unwrap();
```
