# What is this all about?

This is a simple program made to implement the [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) in Rust. 

The actual intent to do this in Rust is to get familar with the language, and also, to try out a simple conjecture that event a baby can do. 

# How to run this?

It's really simple, just install the latest rust `nightly` in your environment and type the following: 

```bash
$ cargo run --release
```

From there, you will see some outputs like this 

```bash
Seed 1 took 200ns
Seed 2 took 156ns
Seed 3 took 482ns
Seed 4 took 107ns
Seed 5 took 370ns
Seed 6 took 361ns
Seed 7 took 454ns
Seed 8 took 235ns
Seed 9 took 693ns
Seed 10 took 544ns
Seed 11 took 450ns
Seed 12 took 2.851µs
Seed 13 took 391ns
Seed 14 took 440ns
Seed 15 took 465ns
Seed 16 took 128ns
Seed 17 took 475ns
Seed 18 took 482ns
Seed 19 took 483ns
Seed 20 took 353ns
...
```

The program will run endelessly (or atleast to the max size of a `usize`). Mind that the objective here is not to prove the conjecture, but rather, a playground to implement and test out some Rust code. 
