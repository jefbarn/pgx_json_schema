use avro_rs::types::Record;
use avro_rs::{Schema, Writer};
use pgx::*;

#[pg_extern]
fn avro_is_valid(schema: JsonB, instance: JsonB) -> bool {
    let parsed_schema =
        Schema::parse(&schema.0).unwrap_or_else(|e| panic!("Error parsing schema: {:?}", e));
    let mut writer = Writer::new(&parsed_schema, Vec::new());

    let val;
    if schema.0["type"] == "record" {
        let mut record = Record::new(&parsed_schema).unwrap();
        for (k, v) in instance.0.as_object().unwrap() {
            record.put(k, avro_rs::types::Value::from(v.clone()))
        }
        val = avro_rs::types::Value::from(record)
    } else {
        val = avro_rs::types::Value::from(instance.0);
    }

    writer.append(val).is_ok()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;
    use std::panic;

    #[pg_test]
    fn test_avro_is_valid_bad_schema() {
        let result = panic::catch_unwind(|| {
            let _valid = Spi::get_one::<bool>(
                r#"
                select avro_is_valid('{
                    "type": "record",
                    "name": "test",
                    "fields": [
                        {"name": "a", "type": "number", "default": 42},
                        {"name": "b", "type": "string"}
                    ]
                }'::jsonb, '{
                    "a": 42,
                    "b": "foo"
                }'::jsonb)"#,
            );
        });
        assert!(result.is_err());
    }

    #[pg_test]
    fn test_avro_is_valid_true() {
        let valid = Spi::get_one::<bool>(
            r#"
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
            }'::jsonb)"#,
        );
        assert_eq!(valid, Some(true))
    }

    #[pg_test]
    fn test_avro_is_valid_false() {
        let valid = Spi::get_one::<bool>(
            r#"
            select avro_is_valid('{
                "type": "record",
                "name": "test",
                "fields": [
                    {"name": "a", "type": "long", "default": 42},
                    {"name": "b", "type": "string"}
                ]
            }'::jsonb, '{
                "a": "27",
                "b": "foo"
            }'::jsonb)"#,
        );
        assert_eq!(valid, Some(false))
    }

    // #[pg_test]
    // fn test_avro_get_errors() {
    //     let (_value, description) = Spi::get_two::<JsonB, String>(
    //         "select * from avro_get_errors('{\"maxLength\": 5}', '\"foobar\"'::jsonb)",
    //     );
    //     assert_eq!(
    //         description,
    //         Some("\"foobar\" is longer than 5 characters".to_string())
    //     )
    // }
}
