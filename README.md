
```
vim -p README.md Cargo.toml ./src/main.rs

apt-get install rustc
apt-get install cargo

cargo clean
cargo build
cargo run

Hello World!

cargo build --release

 ./target/release/my-demo
Hello World!

```


----


iron 
  - drag- and drop, no middleware router, but plugable,

nickle.rs 
  - node express-like, has url router.


cargo search iron

#### resources

http://doc.crates.io/guide.html

```
Note, the ability to put explicit git dependencies,

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```
