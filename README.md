# Blockchain example

This is a Rust implementation of a simple blockchain example based on Daniel van Flymen's blog post [Learn Blockchains by Building One](https://medium.com/p/117428612f46).

The example was written purely out of curiosity and for educational purposes. We advice not to run this in a production setup.

## Prerequisite

- [rust](https://www.rust-lang.org/tools/install) (tested with 1.36.0)

## Running

Start the server locally on port 5000:

`cargo run`

More nodes can be started on separate ports with:

`cargo run -- --p 5001`

## TODO

- add tests
- change `nodes/resolve` endpoint from `.to` -> `.to_async`
- enable CI

## Contribution

Contributions are welcome, feel free to submit a pull request.
