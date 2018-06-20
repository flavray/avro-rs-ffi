use avro_rs::schema::Schema;
use core::AvroStr;

pub struct AvroSchema;

ffi_fn! {
    /// Create an avro schema from its JSON definition.
    unsafe fn avro_schema_from_json(json: *const AvroStr) -> Result<*mut AvroSchema> {
        let schema = Schema::parse_str((&*json).as_str())?;
        Ok(Box::into_raw(Box::new(schema)) as *mut AvroSchema)
    }
}

#[no_mangle]
/// Free an avro schema.
pub unsafe extern "C" fn avro_schema_free(schema: *mut AvroSchema) {
    if !schema.is_null() {
        Box::from_raw(schema as *mut Schema);
    }
}
