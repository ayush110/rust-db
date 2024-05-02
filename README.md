
# rust-db

A simple database written from scratch in Rust. It mimicks SQLite features. 

## Get Started

1. Clone repository
2. `cargo run`
3. Type `.help` to see list of available commands
   
![image](https://github.com/ayush110/rust-db/assets/67121244/c796a62f-335c-499b-9559-a9d33eefbaf0)

### Project Progress

- [ ] REPL interface using Rustyline library
- [ ] Supports `.help`, `.exit` meta commands
- [ ] Parses SQLite SQL dialect to generate AST (Abstract Syntax Tree)
- [ ] Supports `CREATE TABLE`, `INSERT`, `SELECT`
  - [ ] Simple select queries - only single where clause and no joins
- [ ] Standard error handling and validation structure
- [ ] In memory BTreeMap indexes (only for primary keys)
- [ ] Serialization | Deserialization to and from binary encodings
- [ ] Add persistence storage on disk to load db from `.open` command
- [ ] Implement pager
- [ ] Add functionality for rest of meta commands
- [ ] Benchmarking

#### Inspired by and learned from
- https://cstack.github.io/db_tutorial/
- https://github.com/tahmidsadik/simple-db
- https://github.com/joaoh82/rust_sqlite
- https://www.rust-lang.org/learn
