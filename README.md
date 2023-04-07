# countbattle
In this repo are getting two solutions of the problem stated in **[this video](https://youtu.be/U6I-Kwj-AvY)** from the Youtuber **[code_report](https://www.youtube.com/@code_report)** compared. Both are in Rust and you can compare them using the hyperfine command.

## Dependencies
Only hyperfine hast to be installed using cargo, the other dependencies are stated in the Cargo.toml:
```
cargo install hyperfine
```

## Usage

Make sure to compile the release build of the programm before using the hyperfine command:
```
cargo build -r
```
For my Rust solution:
```
hyperfine --warmup 6 -- "target/release/vectest nils -s 69 -l 500000"
```
For the Rust solution from code_report's video:
```
hyperfine --warmup 6 -- "target/release/vectest internet -s 69 -l 500000"
```
For the generated list:
```
hyperfine --warmup 6 --show-output -- "target/release/vectest uncounted -s 69 -l 500000"
```
with -s you can set the seed for the generated list and with -l you can specify the length of the list.
