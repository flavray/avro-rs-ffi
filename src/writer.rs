use avro::writer::Writer;
use schema::AvroSchema;
use avro::schema::Schema;
use core::AvroByteArray;
use codec::AvroCodec;


pub struct AvroWriter;


ffi_fn! {
    /// Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
    unsafe fn avro_writer_new(schema: *const AvroSchema, buffer: *mut AvroByteArray, codec: AvroCodec) -> Result<*mut AvroWriter> {
        let schema = &*(schema as *const Schema);
        let buffer = (&mut *buffer).to_vec_u8();
        let codec = codec.to_codec();
        let writer = Writer::with_codec(schema, buffer, codec);
        Ok(Box::into_raw(Box::new(writer)) as *mut AvroWriter)
    }
}
