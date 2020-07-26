# Rate My Bistro: Server

This MircoService provides a REST API to serve the `Rate My Bistro: App`.

## 1 Architecture Decision Records

| No | Record                                          |
| -- | :---------------------------------------------- |
| 1  | Use Rust to implement the Server                |
| 2  | Use Actix as Service Framework                  |
| 3  | Use ArangoDb for Persistence                    |
| 4  | Separate Service into Contract, Dao and Service |
| 5  | TODO AUTH IMPL                                  |

## 2 Prerequisites

The development on this project requires Rust to be installed:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## 3 Usage

Build the server by running `cargo build` and then run it via `cargo run --bin bistro-service`.

## 4 Structure

| Path             | Core Functionality                     |
| :--------------- | :------------------------------------- |
| bistro-contract  | Provides the SOA contract for entities |
| bistro-dao       | Used for persistence                   |
| bistro-service   | Used to serve the app                  |

## 5 Open Topics

1. Actix-Swagger is not ready for usage yet
    * No support for Arrays
    * Docs how  to serve this output incomplete

--> Let's just implement the api without swagger for now

## 6 Contribution

Before you start contributing, read the following infos:

1. Please document any new code
2. Express changes in semantic commit messages
3. Align your changes with the existing coding style
4. Better ask first and then start changing
5. Use Templates
6. Read the [Code of Conduct](./CODE_OF_CONDUCT.md)

You found a bug somewhere in the code?

--> Open an Issue

You fixed a bug somewhere in the code?

--> Open a pull request

You got an awesome idea to improve the project? You hate your Bistro as much as I do and want to speed up development?

--> The best way to support me in this project starts with a direct contact.
Just send me an email and we will figure out a way on how to split up work :)

--> ansgar.sa@gmail.com

