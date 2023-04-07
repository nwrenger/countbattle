# countbattle
In this repo are getting two solutions of the problem stated in **[this video](https://youtu.be/U6I-Kwj-AvY)** from the Youtuber **[code_report](https://www.youtube.com/@code_report)** compared. Both are in Rust and you can compare them using the hyperfine command. 

Which will be faster?!?!

## Dependencies
Only hyperfine has to be installed using cargo, the other dependencies are stated in the Cargo.toml:
```
cargo install hyperfine
```
Alternatively hyperfine can also be installed with a package manager e.g. `sudo pacman -Syu hyperfine`.

## Usage

Make sure to compile the release build of the programm before using the hyperfine command:
```
cargo build -r
```
For my Rust solution:
```
hyperfine --warmup 6 -- "target/release/countbattle nils -s 69 -l 500000"
```
For the Rust solution from code_report's video:
```
hyperfine --warmup 6 -- "target/release/countbattle internet -s 69 -l 500000"
```
The Makefile should execute all the above commands (if rust is installed)
```
make test-all
```
The shell script only executes the test cases. The first argument to the script is the seed, the second the length of the
test array 
```
./test-all.sh 69 5000000
```
For the generated list(print!):
```
hyperfine --warmup 6 --show-output -- "target/release/countbattle uncounted -s 69 -l 500000"
```
with -s you can set the seed for the generated list and with -l you can specify the length of the list.

And who wins?!?! Compare them yourself!
