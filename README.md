## pgx_json_schema

A JSON Schema validator for Postgres implemented in Rust

This repo is a lightweight connection between the following excellent packages:
* PGX framework for developing PostgreSQL extensions in Rust
  
  https://github.com/zombodb/pgx
* jsonschema-rs Rust schema validation library
  
  https://github.com/Stranger6667/jsonschema-rs

Supported drafts:

* Draft 7 (except optional idn-hostname.json test case)
* Draft 6
* Draft 4 (except optional bignum.json test case)

### Installation:

```shell
# Install Rust
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PGX
cargo install cargo-pgx
cargo pgx init

# Download this repo
curl -L 'https://github.com/jefbarn/pgx_json_schema/archive/refs/tags/0.1.0.tar.gz' \
   | tar -xz --strip-components=1
   
# Build and install the extension package
cargo pgx package

# Enable the extension in your database
create extension pgx_json_schema;
```

### How to use:

```
select * from json_schema_is_valid('{"maxLength": 5}'::jsonb, '"foobar"'::jsonb);

json_schema_is_valid
----------------------
f
```


```
select * from json_schema_get_errors('{"maxLength": 5}'::jsonb, '"foobar"'::jsonb);

error_value |             description              |        details         | instance_path | schema_path
------------+--------------------------------------+------------------------+---------------+-------------
"foobar"    | "foobar" is longer than 5 characters | MaxLength { limit: 5 } |               | /maxLength
```


### Things left to do:

- [ ] Use shared memory to store compiled validator (potential performance gain)
- [ ] More testing
- [ ] Benchmarking
- [ ] Add more schema types like [JTD](https://jsontypedef.com/) and [Avro](https://avro.apache.org/)
- [ ] Support newer JSON Schema drafts
- [x] Add Dockerfile with installation example

### Prior Art
- https://github.com/gavinwahl/postgres-json-schema
- https://github.com/furstenheim/is_jsonb_valid