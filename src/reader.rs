use avro_rs::schema::Schema;
use avro_rs::Reader;
use avro_utils::pickle_value_from_avro;
use core::AvroByteArray;
use schema::AvroSchema;
use serde_pickle::ser::value_to_vec;
use std::ptr;
use types::AvroValue;

pub struct AvroReader;

ffi_fn! {
    /// Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
    unsafe fn avro_reader_new(
        buffer: *const AvroByteArray,
        schema: Option<*const AvroSchema>
    ) -> Result<*mut AvroReader> {
        let reader = match schema {
            None => Reader::new((&*buffer).as_slice())?,
            Some(s) => Reader::with_schema(&*(s as *const Schema), (&*buffer).as_slice())?,
        };
        Ok(Box::into_raw(Box::new(reader)) as *mut AvroReader)
    }
}

ffi_fn! {
    /// Read the next chunk of data out of an avro reader.
    unsafe fn avro_reader_read_next(reader: *mut AvroReader) -> Result<AvroByteArray> {
        let reader = &mut *(reader as *mut Reader<&[u8]>);
        match reader.next() {
            None => Ok(AvroByteArray::default()),
            Some(v) => Ok(AvroByteArray::from_vec_u8(value_to_vec(
                &pickle_value_from_avro(v?),
                false
            )?)),
        }
    }
}

ffi_fn! {
    /// Read the next chunk of data out of an avro reader.
    unsafe fn avro_reader_read_next2(reader: *mut AvroReader) -> Result<*mut AvroValue> {
        let reader = &mut *(reader as *mut Reader<&[u8]>);
        match reader.next() {
            None => Ok(ptr::null_mut()),
            Some(v) => Ok(Box::into_raw(Box::new(v?)) as *mut AvroValue),
        }
    }
}

ffi_fn! {
    /// Free an avro reader. Does NOT free the buffer the reader reads from.
    unsafe fn avro_reader_free(reader: *mut AvroReader) {
        if !reader.is_null() {
            Box::from_raw(reader as *mut Reader<&[u8]>);
        }
    }
}
