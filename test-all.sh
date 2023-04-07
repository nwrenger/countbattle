#/bin/bash

hyperfine --warmup 6 -- "target/release/countbattle nils -s $1 -l $2"
hyperfine --warmup 6 -- "target/release/countbattle internet -s $1 -l $2"