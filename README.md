# transfer.rs
Transfer.sh written in Rust (Client)

---
### Installation:
1. Download the file in Release.
2. Put it into /usr/local/bin/

Or using `cargo build --release` to generate file.

---
### Usage:
```
transfer -f <FILE_PATH> -n <FILE_NAME>
```

---
### Feature:
- [x] Basic upload file
- [ ] Encrypt file and upload

---
### Acknowledgements:
- [Clap](https://github.com/clap-rs/clap)
- [Reqwest](https://github.com/seanmonstar/reqwest)
- [Tokio](https://github.com/tokio-rs/tokio)
