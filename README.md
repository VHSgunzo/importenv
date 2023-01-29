# importenv
Launching an executable file with environment variables from a specific process id

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/importenv.git && cd importenv
```

* **Сompile a binary**
```
rustup default nightly
rustup target add x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly
cargo build --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/importenv/releases)

* **Usage**
```
./importenv $PID {command} {command args}
```
