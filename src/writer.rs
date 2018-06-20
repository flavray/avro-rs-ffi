use avro_rs::schema::Schema;
use avro_rs::types::Value;
use avro_rs::Writer;
use avro_utils::avro_value_from_pickle;
use codec::AvroCodec;
use core::AvroByteArray;
use schema::AvroSchema;
use serde_pickle;
use types::AvroValue;

pub struct AvroWriter;

ffi_fn! {
    /// Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
    unsafe fn avro_writer_new(
        schema: *const AvroSchema,
        codec: AvroCodec
    ) -> Result<*mut AvroWriter> {
        let schema = &*(schema as *const Schema);
        let codec = codec.to_codec();
        let writer = Writer::with_codec(schema, Vec::new(), codec);
        Ok(Box::into_raw(Box::new(writer)) as *mut AvroWriter)
    }
}

ffi_fn! {
    /// Append a pickled avro value to an avro writer. Writing is not necessarily happening here.
    /// Call `avro_writer_flush` to force an actual write.
    unsafe fn avro_writer_append(
        writer: *mut AvroWriter,
        value: *const AvroByteArray
    ) -> Result<usize> {
        let writer = &mut *(writer as *mut Writer<Vec<u8>>);
        let pickle = serde_pickle::from_slice((&*value).as_slice());
        let value = avro_value_from_pickle(writer.schema(), pickle?);
        Ok(writer.append(value?)?)
    }
}

ffi_fn! {
    unsafe fn avro_writer_append2(writer: *mut AvroWriter, value: *mut AvroValue) -> Result<usize> {
        let writer = &mut *(writer as *mut Writer<Vec<u8>>);
        let value = *(Box::from_raw(value as *mut Value));
        Ok(writer.append(value)?)
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
        let buf = (*writer).into_inner();
        Ok(AvroByteArray::from_vec_u8(buf))
    }
}
