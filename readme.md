# rusty-huffman-unicode

## About
Huffman-Compression-in-Rust
A simple command line tool to compress file via Huffman Algorithm.

## Try it! (How to use it?)
- Clone directory to local 
```shell
$ git clone https://github.com/Mega-Pan/Huffman-Compresson-in-Rust && cd Huffman-Compresson-in-Rust
```

- Encode(Compress):
```shell
$ cargo run -- -c <FILE>
```

- Decode(Extract): 
```shell
$ cargo run -- -d <FILE>
```

## Constrains
- Only works on files with pure words
- Files containing null byte (ie. \0) will not work
- Files with over 128 distinct characters might not work
