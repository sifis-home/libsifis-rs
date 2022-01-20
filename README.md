# SIFIS-Home abstraction library

SIFIS-Augmented [WoT](https://www.w3.org/TR/wot-thing-description) implementation.

## How to retrieve all SIFIS Thing information

Run a `SIFIS Thing` locally

``` sh
# Clone the repository containing the SIFIS Thing
$ git clone https://github.com/sifis-home/webthing-rust.git -b sifis-vocabulary-example

# Enter the directory
$ cd webthing-rust

# Run the SIFIS Thing locally
$ cargo run --example single-thing-sifis
```

In another terminal, run the process that retrieves all SIFIS Thing information.

``` sh
# Run process to retrieve all SIFIS Thing information
$ cargo run --example ls
```

## TODO
- [ ] Write the Consumer-focused API
- [ ] Write the Thing-focused API
- [ ] Write more usage examples
- [ ] Use thiserror instead of anyhow (once we can reason on what can go wrong)
- [ ] webthings-set
- [ ] webthings-get


