use std::collections::HashMap;

use failure::{Error, err_msg};

use avro::schema::{RecordSchema, Schema};
use avro::types::{ToAvro, Value};
use serde_pickle::value::HashableValue;
use serde_pickle::value::Value as PickleValue;

pub fn avro_value_from_pickle(schema: &Schema, value: PickleValue) -> Result<Value, Error> {
    match schema {
        &Schema::Null => from_null(value),
        &Schema::Boolean => from_boolean(value),
        &Schema::Int => from_int(value),
        &Schema::Long => from_long(value),
        &Schema::Float => from_float(value),
        &Schema::Double => from_double(value),
        &Schema::Bytes => from_bytes(value),
        &Schema::String => from_string(value),
        &Schema::Fixed { size, .. } => from_fixed(size, value),
        &Schema::Array(ref inner) => from_array(inner, value),
        &Schema::Map(ref inner) => from_map(inner, value),
        &Schema::Union(ref inner) => from_union(inner, value),
        &Schema::Record(ref rschema) => from_record(rschema, value),
        &Schema::Enum { .. } => Err(err_msg("enum not yet supported")),
    }
}

fn from_null(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::None => Ok(Value::Null),
        _ => Err(err_msg("not a null")),
    }
}

fn from_boolean(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::Bool(b) => Ok(Value::Boolean(b)),
        _ => Err(err_msg("not a bool")),
    }
}

fn from_int(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::I64(n) => Ok(Value::Int(n as i32)),
        _ => Err(err_msg("not an int")),
    }
}

fn from_long(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::I64(n) => Ok(Value::Long(n)),
        _ => Err(err_msg("not a long")),
    }
}

fn from_float(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::F64(x) => Ok(Value::Float(x as f32)),
        _ => Err(err_msg("not a float")),
    }
}

fn from_double(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::F64(x) => Ok(Value::Double(x)),
        _ => Err(err_msg("not a double")),
    }
}

fn from_bytes(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::Bytes(bytes) => Ok(Value::Bytes(bytes)),
        PickleValue::String(s) => Ok(Value::Bytes(s.into_bytes())),
        _ => Err(err_msg("not a bytes")),
    }
}

fn from_string(value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::String(s) => Ok(Value::String(s)),
        PickleValue::Bytes(bytes) => String::from_utf8(bytes).map_err(|_| err_msg("not a valid utf-8 string")).map(Value::String),
        _ => Err(err_msg("not a string")),
    }
}

fn from_fixed(size: usize, value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::Bytes(bytes) => {
            if size == bytes.len() {
                Ok(Value::Fixed(size, bytes))
            } else {
                Err(err_msg("fixed size does not match"))
            }
        },
        _ => Err(err_msg("not a fixed")),
    }
}

fn from_array(schema: &Schema, value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::List(values) | PickleValue::Tuple(values) => Ok(Value::Array(
            values.into_iter()
                .map(|value| avro_value_from_pickle(schema, value))
                .collect::<Result<Vec<_>, _>>()?)),
        PickleValue::Set(values) | PickleValue::FrozenSet(values) => Ok(Value::Array(
            values.into_iter()
                .map(|value| avro_value_from_pickle(schema, value.into_value()))
                .collect::<Result<Vec<_>, _>>()?)),
        _ => Err(err_msg("not an array")),
    }
}

fn from_map(schema: &Schema, value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::Dict(values) => Ok(Value::Map(
            values.into_iter()
                .map(|(key, value)| {
                    if let HashableValue::String(key) = key {
                        let value = avro_value_from_pickle(schema, value)?;
                        Ok((key, value))
                    } else {
                        Err(err_msg("map key should be string"))
                    }
                })
                .collect::<Result<HashMap<_, _>, _>>()?)),
        _ => Err(err_msg("not a map")),
    }
}

fn from_union(schema: &Schema, value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::None => Ok(Value::Union(None)),
        value => Ok(Value::Union(Some(Box::new(avro_value_from_pickle(schema, value)?)))),
    }
}

fn from_record(rschema: &RecordSchema, value: PickleValue) -> Result<Value, Error> {
    match value {
        PickleValue::Dict(mut fields) => {
            Ok(Value::Record(rschema.fields.iter()
                .map(|field| {
                    let value = match fields.remove(&HashableValue::String(field.name.clone())) {
                        Some(value) => avro_value_from_pickle(&field.schema, value),
                        None => match field.default {
                            Some(ref value) => Ok(value.clone().avro()),
                            None => Err(err_msg(format!("missing field {} in record", field.name))),
                        }
                    };

                    value.map(|value| (field.name.clone(), value))
                })
                .collect::<Result<Vec<_>, _>>()?))
        },
        _ => Err(err_msg("not a record")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn pickle_int() {
        let avro_value = avro_value_from_pickle(&Schema::Int, PickleValue::I64(42));
        assert!(avro_value.is_ok());
        assert_eq!(avro_value.unwrap(), Value::Int(42));
    }

    #[test]
    fn pickle_record() {
        let schema = Schema::parse_str(r#"
        {"namespace": "test", "type": "record", "name": "Test", "fields": [{"type": {"type": "string"}, "name": "field"}]}
        "#).unwrap();

        let mut record = BTreeMap::new();
        record.insert(HashableValue::String("field".to_owned()), PickleValue::String("foo".to_owned()));

        let avro_value = avro_value_from_pickle(&schema, PickleValue::Dict(record));
        if let Ok(Value::Record(fields)) = avro_value {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].1, Value::String("foo".to_owned()));
        } else {
            assert!(false);
        }
    }
}
