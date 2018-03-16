extern crate avro;
extern crate failure;
extern crate serde;
extern crate serde_pickle;

#[macro_use]
mod utils;

mod core;
mod schema;
mod writer;
mod reader;
mod codec;
mod avro_utils;

pub use core::*;
pub use schema::*;
pub use codec::*;
pub use writer::*;
pub use reader::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

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
                _ => println!("{:?}", value),
            }
        }
        unsafe { avro_reader_free(reader) };
        unsafe { avro_schema_free(schema) };
    }
}
