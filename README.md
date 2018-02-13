How to run:

1. Install [afl.rs](https://rust-fuzz.github.io/book/afl/setup.html)
2. `cargo afl build --release`
3. `cargo afl fuzz -m1000 -t1000+ -i in -o out -M 001 target/release/serde_yaml_fuzz`


