# Semester Short Report

So, the levenshtein thingy doesn't work.

New idea: create vec of words and their counts in the docs and do something with that.
and do something with the trigrams.

## Build and run from source

1. Install [`rupstup`](https://rustup.rs/) on your system.
Go to the website and follow the instructions on there.

2. Make sure that `cargo` is in your path environment variable.
you can verify this by opening a console and running:
~~~bash
cargo --version
~~~
If the output is something like `cargo 1.xx.x (hash)` then you've done it right.

3. To run the project run the following command in the project root:
~~~bash
cargo run --release
~~~
