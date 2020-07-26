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

TODO ``How to install Rust``

## 3 Usage

Build the server by running `cargo build` and then run it via `cargo run --bin bistro-service`.


