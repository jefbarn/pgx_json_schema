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

Bonus support added for:
* [JSON Type Definition (JTD)](https://jsontypedef.com/)
* [Apache Avro](https://avro.apache.org/)

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

#### JSON Schema
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

#### JSON Type Definition
```
select jtd_is_valid('{
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "uint32" },
        "phones": {
            "elements": {
                "type": "string"
            }
        }
    }
}'::jsonb, '{
    "age": "43",
    "phones": ["+44 1234567", 442345678]
}'::jsonb);

 jtd_is_valid 
--------------
 f
```

```
select instance_path, schema_path from jtd_get_errors('{
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "uint32" },
        "phones": {
            "elements": {
                "type": "string"
            }
        }
    }
}', '{
    "age": "43",
    "phones": ["+44 1234567", 442345678]
}'::jsonb);

 instance_path |           schema_path            
---------------+----------------------------------
 /age          | /properties/age/type
               | /properties/name
 /phones/1     | /properties/phones/elements/type

```

> **_NOTE:_**  The jtd library only reports the position of the validation errors, not a description.

#### Apache Avro
```
select avro_is_valid('{
    "type": "record",
    "name": "test",
    "fields": [
        {"name": "a", "type": "long", "default": 42},
        {"name": "b", "type": "string"}
    ]
}'::jsonb, '{
    "a": 27,
    "b": "foo"
}'::jsonb);

 avro_is_valid 
---------------
 t
```

> **_NOTE:_**  The avro library only does complete validation, there is no way to list the errors.


### Things left to do:

- [ ] Use shared memory to store compiled validator (potential performance gain)
- [ ] More testing
- [ ] Benchmarking
- [x] Add more schema types like [JTD](https://jsontypedef.com/) and [Avro](https://avro.apache.org/)
- [ ] Support newer JSON Schema drafts
- [x] Add Dockerfile with installation example

### Prior Art
- https://github.com/gavinwahl/postgres-json-schema
- https://github.com/furstenheim/is_jsonb_valid