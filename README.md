Quick'n'dirty minimal reproducible example for [rust-htslib/issues/#401](https://github.com/rust-bio/rust-htslib/issues/401)

# Usage 

```bash
cargo run --quiet -- data/reference.fa > /dev/null & watch "ps -u --pid $!"; kill -9 $!
```
