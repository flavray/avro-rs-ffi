#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
extern crate avro_rs;
extern crate failure;
extern crate serde;
extern crate serde_pickle;

#[macro_use]
mod utils;

mod avro_utils;
mod codec;
mod core;
mod reader;
mod schema;
mod types;
mod writer;

pub use codec::*;
pub use core::*;
pub use reader::*;
pub use schema::*;
pub use types::*;
pub use writer::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_pickle::value::Value as PickleValue;
    use std::collections::HashMap;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn full_test() {
        let json = CString::new(r#"
        {"namespace": "test", "type": "record", "name": "Test", "fields": [{"type": {"type": "string"}, "name": "field"}]}
        "#).unwrap();
        let avro_json = unsafe { core::avro_str_from_c_str(json.as_ptr()) };
        let schema = unsafe { schema::avro_schema_from_json(&avro_json) };
        assert!(!schema.is_null());

        let writer = unsafe { writer::avro_writer_new(schema, AvroCodec::Null) };
        assert!(!writer.is_null());
        let pickle = b"(dp1\nS'field'\np2\nS'foo'\np3\ns.";
        let write_buffer = core::AvroByteArray::from_vec_u8(pickle.to_vec());
        let _done = unsafe { writer::avro_writer_append(writer, &write_buffer) };
        let _flushed = unsafe { writer::avro_writer_flush(writer) };
        let data = unsafe { writer::avro_writer_into_data(writer) };

        let reader = unsafe { reader::avro_reader_new(&data, None) };
        loop {
            let value = unsafe { reader::avro_reader_read_next(reader) };
            match value.len {
                0 => break,
                _ => {
                    let pickle_map = serde_pickle::from_slice::<HashMap<String, PickleValue>>(
                        value.as_slice(),
                    ).unwrap();
                    let pickle_value = pickle_map.get("field").unwrap();
                    if let PickleValue::String(s) = pickle_value {
                        assert_eq!("foo", s);
                    } else {
                        assert!(false, "the value is not a string");
                    }
                },
            }
        }
        unsafe { avro_reader_free(reader) };
        unsafe { avro_schema_free(schema) };
    }

    #[test]
    fn full_test2() {
        unsafe {
            let json = CString::new(r#"
            {"namespace": "test", "type": "record", "name": "Test", "fields": [{"type": {"type": "string"}, "name": "field"}]}
            "#).unwrap();
            let avro_json = core::avro_str_from_c_str(json.as_ptr());
            let schema = schema::avro_schema_from_json(&avro_json);
            assert!(!schema.is_null());

            let writer = writer::avro_writer_new(schema, AvroCodec::Null);
            assert!(!writer.is_null());

            let field_str = CString::new("field").unwrap();
            let field = core::avro_str_from_c_str(field_str.as_ptr());

            let foo_str = CString::new("foo").unwrap();
            let foo = core::avro_str_from_c_str(foo_str.as_ptr());
            let foo_value = types::avro_value_string_new(foo);

            let record = types::avro_record_new(schema);
            let _put = types::avro_record_put(record, &field, foo_value);
            let value = types::avro_record_to_value(record);

            let _done = writer::avro_writer_append2(writer, value);
            let _flushed = writer::avro_writer_flush(writer);
            let data = writer::avro_writer_into_data(writer);

            let reader = reader::avro_reader_new(&data, None);
            let read_value = reader::avro_reader_read_next2(reader);
            assert!(!read_value.is_null());
            assert_eq!(reader::avro_reader_read_next2(reader), ptr::null_mut());

            let string_value = types::avro_value_record_get(read_value, &field);
            let internal_string = types::avro_value_string_get(string_value);
            assert_eq!("foo", internal_string.as_str());

            avro_reader_free(reader);
            avro_schema_free(schema);
            avro_value_free(read_value);
        }
    }
}
