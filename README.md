# Comparison of mongo drivers

### Rust
```console
▶ cargo run --release
   Compiling mongobenchmark v0.1.0 (/Users/ericjolibois/os/mongo-read-write-benchmark/rust)
    Finished release [optimized] target(s) in 57.37s
     Running `target/release/mongobenchmark`
Insert time for 1000000 docs: 4.42s
Read time for 1000000 docs: 1.16s
Amount of found books: 333559
```

### Python
```console
▶ python main.py
Insert time for 1000000 docs: 8.92s
Read time for 1000000 docs: 2.56s
Amount of found books: 333192
```

### Go
```console
▶ go run main.go
Insert time for 1000000 docs: 4.55s
Read time for 1000000 docs: 1.20s
Amount of found books: 333459
```
