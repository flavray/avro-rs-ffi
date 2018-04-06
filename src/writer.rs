use std::collections::HashMap;

use failure::err_msg;

use avro::writer::Writer;
use schema::AvroSchema;
use avro::schema::Schema;
use avro::types::Value;
use core::{AvroByteArray, AvroStr};
use codec::AvroCodec;
use avro_utils::avro_value_from_pickle;
use serde_pickle;

pub struct AvroPValue;
pub struct AvroWriter;

ffi_fn! {
    unsafe fn avro_value_null() -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::Null)) as *mut AvroPValue)
    }
}

ffi_fn! {
    unsafe fn avro_value_boolean(b: i32) -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::Boolean(b == 1))) as *mut AvroPValue)
    }
}

ffi_fn! {
    unsafe fn avro_value_long(n: i64) -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::Long(n))) as *mut AvroPValue)
    }
}

ffi_fn! {
    unsafe fn avro_value_double(x: f64) -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::Double(x))) as *mut AvroPValue)
    }
}

// TODO: bytes

ffi_fn! {
    unsafe fn avro_value_string(s: AvroStr) -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::String(s.into_string()))) as *mut AvroPValue)
    }
}


ffi_fn! {
    unsafe fn avro_value_map(capacity: usize) -> Result<*mut AvroPValue> {
        Ok(Box::into_raw(Box::new(Value::Map(HashMap::with_capacity(capacity)))) as *mut AvroPValue)
    }
}

ffi_fn! {
    unsafe fn avro_map_put(map: *mut AvroPValue, key: AvroStr, value: *mut AvroPValue) -> Result<()> {
        let map = &mut *(map as *mut Value);

        match map {
            &mut Value::Map(ref mut items) => {
                let key = key.into_string();
                let value = Box::from_raw(&mut *(value as *mut Value));

                items.insert(key, *value);
                Ok(())
            },
            _ => {
                println!("noes");
                Err(err_msg("Noes"))
            },
        }
    }
}

/*
ffi_fn! {
    /// Create an avro record
    unsafe fn avro_value_new(schema: *const AvroSchema) -> Result<*mut AvroValueS> {
        let value = AvroValue::default();
        let avro_value = AvroValueS {
            value: Box::into_raw(Box::new(value)) as *const AvroValue,
            schema: schema,
            owned: true,
        };
        Ok(Box::into_raw(Box::new(avro_value)) as *mut AvroValueS)
    }
}

ffi_fn! {
    /// Create an avro record
    unsafe fn avro_value_map(schema: *const AvroSchema, len: usize) -> Result<*mut AvroValueS> {
        let schema = &*(schema as *const Schema);
        let value = match schema {
            // &Schema::Map(ref inner) => AvroValue { map: }
            &Schema::Record(ref rschema) => AvroValue { record: }
        }
        let value = AvroValue::default();
        let avro_value = AvroValueS {
            value: Box::into_raw(Box::new(value)) as *const AvroValue,
            schema: schema,
            owned: true,
        };
        Ok(Box::into_raw(Box::new(avro_value)) as *mut AvroValueS)
    }
}

ffi_fn! {
    /// Create an avro record
    unsafe fn avro_record_put(record: *mut AvroRecord, field: *const AvroByteArray, value: *const AvroValue) -> Result<()> {
        let record = &mut *(record as *mut Record);
        let field = str::from_utf8((*field).as_slice())?;

        match *value {
            AvroValue { null } => record.put(field, Value::Null),
            AvroValue { boolean: b } => record.put(field, Value::Boolean(b)),
            AvroValue { int: n } => record.put(field, Value::Int(n)),
            AvroValue { long: n } => record.put(field, Value::Long(n)),
            AvroValue { float: x } => record.put(field, Value::Float(x)),
            AvroValue { double: x } => record.put(field, Value::Double(x)),
            AvroValue { bytes: bytes } => record.put(field, Value::Bytes((*bytes).to_vec_u8())),
            AvroValue { string: string } => {
                let string = String::from_utf8((*string).to_vec_u8())?;
                record.put(field, Value::String(string))
            },
            AvroValue { fixed: fixed } => record.put(field, Value::Fixed((*fixed).len, (*fixed).to_vec_u8())),
            AvroValue { union: union } => record.put(field, Value::Null),  // TODO
            AvroValue { array: array } => record.put(field, Value::Null),
            AvroValue { map: map } => record.put(field, Value::Null),
        }

        Ok(())
    }
}
*/

ffi_fn! {
    /// Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
    unsafe fn avro_writer_new(schema: *const AvroSchema, codec: AvroCodec) -> Result<*mut AvroWriter> {
        let schema = &*(schema as *const Schema);
        let codec = codec.to_codec();
        let writer = Writer::with_codec(schema, Vec::new(), codec);
        Ok(Box::into_raw(Box::new(writer)) as *mut AvroWriter)
    }
}

ffi_fn! {
    /// Append a pickled avro value to an avro writer. Writing is not necessarily happening here.
    /// Call `avro_writer_flush` to force an actual write.
    unsafe fn avro_writer_append(writer: *mut AvroWriter, value: *const AvroByteArray) -> Result<usize> {
        let writer = &mut *(writer as *mut Writer<Vec<u8>>);
        let pickle = serde_pickle::from_slice((&*value).as_slice());
        let value = avro_value_from_pickle(writer.schema(), pickle?);
        Ok(writer.append(value?)?)
    }
}

ffi_fn! {
    /// Append a pickled avro value to an avro writer. Writing is not necessarily happening here.
    /// Call `avro_writer_flush` to force an actual write.
    unsafe fn avro_writer_append2(writer: *mut AvroWriter, value: *const AvroPValue) -> Result<usize> {
        let writer = &mut *(writer as *mut Writer<Vec<u8>>);
        let value = &mut *(value as *mut Value);
        let value = value.clone().resolve(writer.schema());  // TODO: not clone?
        Ok(writer.append(value?)?)
    }
}

ffi_fn! {
    /// Flush an avro writer.
    unsafe fn avro_writer_flush(writer: *mut AvroWriter) -> Result<usize> {
        let writer = &mut *(writer as *mut Writer<Vec<u8>>);
        Ok(writer.flush()?)
    }
}

ffi_fn! {
    /// Consume an avro writer and return the avro serialized data.
    unsafe fn avro_writer_into_data(writer: *mut AvroWriter) -> Result<AvroByteArray> {
        let writer = Box::from_raw(writer as *mut Writer<Vec<u8>>);
        Ok(AvroByteArray::from_vec_u8((*writer).into_inner()))
    }
}
