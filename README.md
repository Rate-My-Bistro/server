# Rate My Bistro: Server

This Service provides an API to serve the `Rate My Bistro: App`.
In more detail, it lets users manage their account, requests bistro menus
and accepts additional information for these menus (e.g. a rating).

## 1 Architecture Decision Records

The following decision were made with the Rate-My-Bistro team.
The Api Server has to comply with these records at any time.

| No | Record                                                |
| -- | :---------------------------------------------------- |
| 1  | Use Rust to implement the Server                      |
| 2  | Use Rocket.rs as Service Framework                    |
| 3  | Use ArangoDb for Persistence                          |
| 4  | Separate Service into domains                         |
| 5  | Use behavior driven testing and spare with unit tests |
| 6  | Decouple tests from web framework (avoid lock in)     |
| 7  | TODO AUTH                                             |


## 2 Prerequisites

The development on this project requires Rust to be installed:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

As the rocket framework requires an unstable rust version, the
following steps are required, too:
```
rustup toolchain install nightly
rustup default nightly
```

## 3 Usage

The server provides you the following options to operate:

1. Use cargo: ``cargo run`` - this will start rocket with the debug configuration
2. Use docker-compose ``docker-compose up server`` [from here](https://github.com/Rate-My-Bistro/infrastructure)

Please note that a running database is required in order to use the server.

## 4 Testing

First, ensure your server (and database) is up and running in an environment of your choice.
As soon as the server can accept requests, you can start the tests by entering the following command:

```
cargo test --test cucumber
```

## 5 Project Structure

| Path            | Core Functionality                        |
| :-------------- | :---------------------------------------- |
| src/middleware/ | Utilities that intercept handled requests |
| src/menu/       | Handlers to provide menu information      |
| tests/          | BDD tests                                 |

## 6 Domains

| Domain          | Core Functionality                           |
| :-------------- | :------------------------------------------- |
| Menu            | Represents a meal with a list of side dishes |

## 7 Contribution

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
Just email me, and we will figure out a way on how to split up work :)

--> ansgar.sa@gmail.com

