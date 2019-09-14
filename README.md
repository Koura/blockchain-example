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

## Usage

If you have `jq` installed on your machine, the output can be turned into a more human-readable format by appending 
`| jq '.'` at the end of the commands.

Mine a new block:

`curl http://localhost:5000/mine`

Creating a new transaction:

`curl -H "Content-Type: application/json" --request POST --data '{"sender":"e79fcabd1d70433191701d17c4d13112", "recipient":"some-other-address", "amount":5}' http://localhost:5000/transactions/new`

View the full chain:

`curl http://localhost:5000/chain`

Add a new node:

`curl -H "Content-Type: application/json" --request POST --data '{"nodes":["http://localhost:5001"]}' http://localhost:5000/nodes/register`

Reaching consensus with other registered nodes in the network:

`curl http://localhost:5000/nodes/resolve`

## TODO

- add tests
- change `nodes/resolve` endpoint from `.to` -> `.to_async`
- enable CI

## Contribution

Contributions are welcome, feel free to submit a pull request.
