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