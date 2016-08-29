
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
https://github.com/redox-os/redox
https://www.reddit.com/r/rust/comments/4gor9j/is_rust_web_ready_yet_iron_and_nickel_for/
https://github.com/flosse/rust-web-framework-comparison


https://github.com/iron/iron/                            240 forks
https://github.com/nickel-org/nickel.rs/                 123 forks

iron router,
https://github.com/iron/router

static file,
https://github.com/iron/staticfile


Think of hyper as libhttp (just unofficially). Iron and Nickel both use it to handle the server protocol. 

```
Note, the ability to put explicit git dependencies,

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```


####  functional

https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html

#### Vim

git clone https://github.com/rust-lang/rust.vim
ln -s /home/meteo/rust.vim/indent/rust.vim /usr/share/vim/vim74/indent/
ln -s /home/meteo/rust.vim/syntax/rust.vim /usr/share/vim/vim74/syntax/

:setf rust

