extern crate failure;
extern crate avro;
extern crate serde;

#[macro_use] mod utils;

mod core;
mod schema;
mod writer;
mod reader;
mod codec;

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
    fn create_schema() {
        let json = CString::new(r#"
        {"namespace": "test", "type": "record", "name": "Test", "fields": [{"type": {"type": "string"}, "name": "field"}]}
        "#).unwrap();
        let avro_json = unsafe {core::avro_str_from_c_str(json.as_ptr()) };
        let schema = unsafe {schema::avro_schema_from_json(&avro_json) };
        assert!(!schema.is_null());
        unsafe { avro_schema_free(schema) };
    }
}
