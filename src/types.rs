use avro_rs::schema::Schema;
use avro_rs::types::ToAvro;
use avro_rs::types::{Record, Value};
use core::{AvroByteArray, AvroStr};
use failure::err_msg;
use schema::AvroSchema;
use std::collections::HashMap;
use std::os::raw::{c_double, c_float, c_int, c_long};

pub struct AvroRecord;
pub struct AvroValue;

#[no_mangle]
pub unsafe extern "C" fn avro_value_free(v: *mut AvroValue) {
    if !v.is_null() {
        Box::from_raw(v);
    }
}

#[no_mangle]
pub unsafe extern "C" fn avro_record_free(r: *mut AvroRecord) {
    if !r.is_null() {
        Box::from_raw(r as *mut Record);
    }
}

ffi_fn! {
    unsafe fn avro_value_null_new() -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Null))
    }
}

ffi_fn! {
    unsafe fn avro_value_null_get(value: *const AvroValue) -> Result<()> {
        let value = &*(value as *const Value);
        match *value {
            Value::Null => Ok(()),
            _ => Err(err_msg("value is not a null")),
        }
    }
}

ffi_fn! {
    unsafe fn avro_value_boolean_new(b: i32) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Boolean(b == 1)))
    }
}

ffi_fn! {
    unsafe fn avro_value_boolean_get(value: *const AvroValue) -> Result<bool> {
        let value = &*(value as *const Value);
        if let Value::Boolean(b) = *value { Ok(b) } else { Err(err_msg("value is not a bool")) }
    }
}

ffi_fn! {
    unsafe fn avro_value_int_new(n: c_int) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Int(n as i32)))
    }
}

ffi_fn! {
    unsafe fn avro_value_int_get(value: *const AvroValue) -> Result<c_int> {
        let value = &*(value as * const Value);
        if let Value::Int(i) = *value { Ok(i as c_int) } else { Err(err_msg("value is not an int")) }
    }

}

ffi_fn! {
    unsafe fn avro_value_long_new(n: c_long) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Long(n as i64)))
    }
}

ffi_fn! {
    unsafe fn avro_value_long_get(value: *const AvroValue) -> Result<c_long> {
        let value = &*(value as * const Value);
        if let Value::Long(l) = *value { Ok(l as c_long) } else { Err(err_msg("value is not a long")) }
    }
}

ffi_fn! {
    unsafe fn avro_value_float_new(x: c_float) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Float(x as f32)))
    }
}

ffi_fn! {
    unsafe fn avro_value_float_get(value: *const AvroValue) -> Result<c_float> {
        let value = &*(value as * const Value);
        if let Value::Float(d) = *value { Ok(d as c_float) } else { Err(err_msg("value is not a float")) }
    }
}

ffi_fn! {
    unsafe fn avro_value_double_new(x: c_double) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Double(x as f64)))
    }
}

ffi_fn! {
    unsafe fn avro_value_double_get(value: *const AvroValue) -> Result<c_double> {
        let value = &*(value as * const Value);
        if let Value::Double(d) = *value { Ok(d as c_double) } else { Err(err_msg("value is not a double")) }
    }
}

ffi_fn! {
    unsafe fn avro_value_bytes_new(b: AvroByteArray) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Bytes(b.into_vec_u8())))
    }
}

ffi_fn! {
    unsafe fn avro_value_bytes_get(value: *const AvroValue) -> Result<AvroByteArray> {
        let value = &*(value as *const Value);
        if let Value::Bytes(ref b) = *value {
            Ok(AvroByteArray::new(b))
        } else {
            Err(err_msg("value is not bytes"))
        }
    }
}

ffi_fn! {
    unsafe fn avro_value_string_new(s: AvroStr) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::String(s.into_string())))
    }
}

ffi_fn! {
    unsafe fn avro_value_string_get(value: *const AvroValue) -> Result<AvroStr> {
        let value = &*(value as *const Value);
        if let Value::String(ref s) = *value {
            Ok(AvroStr::new(s))
        } else {
            Err(err_msg("value is not a string"))
        }
    }
}

ffi_fn! {
    unsafe fn avro_value_enum_new(value_index: c_int, value_repr: AvroStr) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Enum(value_index as i32, value_repr.into_string())))
    }
}

ffi_fn! {
    unsafe fn avro_value_enum_get(value: *const AvroValue) -> Result<AvroStr> {
        let value = &*(value as * const Value);
        if let Value::Enum(_, ref s) = *value { Ok(AvroStr::new(s)) } else { Err(err_msg("value is not an enum")) }
    }

}

ffi_fn! {
    unsafe fn avro_value_fixed_new(len: usize, b: AvroByteArray) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Fixed(len, b.into_vec_u8())))
    }
}

ffi_fn! {
    unsafe fn avro_value_fixed_get(value: *const AvroValue) -> Result<AvroByteArray> {
        let value = &*(value as *const Value);
        if let Value::Fixed(_, ref b) = *value {
            Ok(AvroByteArray::new(b))
        } else {
            Err(err_msg("value is not fixed"))
        }
    }
}

ffi_fn! {
    unsafe fn avro_value_union_new(value: *mut AvroValue) -> Result<*mut AvroValue> {
        let value = if value.is_null() { None } else { Some(Box::from_raw(value as *mut Value)) };
        Ok(ffi_avro_value!(Value::Union(value)))
    }
}

ffi_fn! {
    unsafe fn avro_value_array_new(capacity: usize) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Array(Vec::with_capacity(capacity))))
    }
}

ffi_fn! {
    unsafe fn avro_array_append(array: *mut AvroValue, value: *mut AvroValue) -> Result<()> {
        let array = &mut *(array as *mut Value);
        match *array {
            Value::Array(ref mut items) => {
                let value = Box::from_raw(value as *mut Value);
                items.push(*value);
                Ok(())
            },
            _ => {
                Err(err_msg("Value is not an array"))
            },
        }
    }
}

ffi_fn! {
    unsafe fn avro_value_map_new(capacity: usize) -> Result<*mut AvroValue> {
        Ok(ffi_avro_value!(Value::Map(HashMap::with_capacity(capacity))))
    }
}

ffi_fn! {
    unsafe fn avro_map_put(map: *mut AvroValue, key: AvroStr, value: *mut AvroValue) -> Result<()> {
        let map = &mut *(map as *mut Value);
        match *map {
            Value::Map(ref mut items) => {
                let key = key.into_string();
                let value = Box::from_raw(value as *mut Value);
                items.insert(key, *value);
                Ok(())
            },
            _ => {
                Err(err_msg("Value is not a map"))
            },
        }
    }
}

ffi_fn! {
    unsafe fn avro_record_new(schema: *const AvroSchema) -> Result<*mut AvroRecord> {
        let schema = &*(schema as *const Schema);
        let record = Record::new(schema).ok_or_else(|| err_msg("record creation failed"))?;
        Ok(Box::into_raw(Box::new(record)) as *mut AvroRecord)
    }
}

ffi_fn! {
    unsafe fn avro_record_put(
        record: *mut AvroRecord,
        field: *const AvroStr,
        value: *mut AvroValue
    ) -> Result<()> {
        let record = &mut *(record as *mut Record);
        let field = (&*field).as_str();
        let value = *(Box::from_raw(value as *mut Value));
        record.put(&field, value); // TODO: CAN'T FAIL???
        Ok(())
    }
}

ffi_fn! {
    unsafe fn avro_record_to_value(record: *mut AvroRecord) -> Result<*mut AvroValue> {
        let record = *(Box::from_raw(record as *mut Record));
        Ok(ffi_avro_value!(record.avro()))
    }
}

ffi_fn! {
    unsafe fn avro_value_record_get(
        record: *const AvroValue,
        field: *const AvroStr
    ) -> Result<*const AvroValue> {
        let field = (&*field).as_str();
        let record = &*(record as *const Value);
        if let Value::Record(ref fields) = *record {
            if let Some(&(_, ref v)) = fields.iter().find(|&&(ref n, _)| n == field) {
                Ok(v as *const Value as *const AvroValue)
            } else {
                Err(err_msg("Field not in record")) // TODO: WHAT ABOUT NULLABLE VALUE? AND OTHER COMPATIBLE SCHEMAS?
            }
        } else {
            Err(err_msg("Value is not a record"))
        }
    }
}
