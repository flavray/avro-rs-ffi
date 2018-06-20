/* c bindings to the avro-rs library */

#ifndef AVRO_RS_BINDINGS_H
#define AVRO_RS_BINDINGS_H

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

/*
 * Replicates avro::Codec enum so we can use a C-compatible representation
 */
typedef enum {
  AVRO_CODEC_NULL,
  AVRO_CODEC_DEFLATE,
  AVRO_CODEC_SNAPPY,
} AvroCodec;

/*
 * C-style error codes
 */
enum AvroErrorCode {
  AVRO_ERROR_CODE_NO_ERROR = 0,
  AVRO_ERROR_CODE_PANIC = 1,
  AVRO_ERROR_CODE_UNKNOWN = 3,
};
typedef uint32_t AvroErrorCode;

typedef struct AvroReader AvroReader;

typedef struct AvroRecord AvroRecord;

typedef struct AvroSchema AvroSchema;

typedef struct AvroValue AvroValue;

typedef struct AvroWriter AvroWriter;

/*
 * Represents a byte array.
 */
typedef struct {
  unsigned char *data;
  uintptr_t len;
  bool owned;
} AvroByteArray;

/*
 * Represents a string.
 */
typedef struct {
  char *data;
  uintptr_t len;
  bool owned;
} AvroStr;

void avro_array_append(AvroValue *array, AvroValue *value);

/*
 * Frees a avro byte array.
 *
 * If the array is marked as not owned then this function does not
 * do anything.
 */
void avro_byte_array_free(AvroByteArray *a);

/*
 * Creates a avro byte array from a c string.
 *
 * This sets the array to owned.  In case it's not owned you either have
 * to make sure you are not freeing the memory or you need to set the
 * owned flag to false.
 */
AvroByteArray avro_byte_array_from_c_array(const unsigned char *a, uintptr_t len);

/*
 * Clears the last error.
 */
void avro_err_clear(void);

/*
 * Returns the panic information as string.
 */
AvroStr avro_err_get_backtrace(void);

/*
 * Returns the last error code.
 *
 * If there is no error, 0 is returned.
 */
AvroErrorCode avro_err_get_last_code(void);

/*
 * Returns the last error message.
 *
 * If there is no error an empty string is returned.  This allocates new memory
 * that needs to be freed with `avro_str_free`.
 */
AvroStr avro_err_get_last_message(void);

/*
 * Initializes the library
 */
void avro_init(void);

void avro_map_put(AvroValue *map, AvroStr key, AvroValue *value);

/*
 * Free an avro reader. Does NOT free the buffer the reader reads from.
 */
void avro_reader_free(AvroReader *reader);

/*
 * Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
 */
AvroReader *avro_reader_new(const AvroByteArray *buffer, const AvroSchema *schema);

/*
 * Read the next chunk of data out of an avro reader.
 */
AvroByteArray avro_reader_read_next(AvroReader *reader);

/*
 * Read the next chunk of data out of an avro reader.
 */
AvroValue *avro_reader_read_next2(AvroReader *reader);

void avro_record_free(AvroRecord *r);

AvroRecord *avro_record_new(const AvroSchema *schema);

void avro_record_put(AvroRecord *record, const AvroStr *field, AvroValue *value);

AvroValue *avro_record_to_value(AvroRecord *record);

/*
 * Free an avro schema.
 */
void avro_schema_free(AvroSchema *schema);

/*
 * Create an avro schema from its JSON definition.
 */
AvroSchema *avro_schema_from_json(const AvroStr *json);

/*
 * Frees a avro str.
 *
 * If the string is marked as not owned then this function does not
 * do anything.
 */
void avro_str_free(AvroStr *s);

/*
 * Creates a avro str from a c string.
 *
 * This sets the string to owned. In case it's not owned you either have
 * to make sure you are not freeing the memory or you need to set the
 * owned flag to false.
 */
AvroStr avro_str_from_c_str(const char *s);

AvroValue *avro_value_array_new(uintptr_t capacity);

bool avro_value_boolean_get(const AvroValue *value);

AvroValue *avro_value_boolean_new(int32_t b);

AvroByteArray avro_value_bytes_get(const AvroValue *value);

AvroValue *avro_value_bytes_new(AvroByteArray b);

double avro_value_double_get(const AvroValue *value);

AvroValue *avro_value_double_new(double x);

AvroStr avro_value_enum_get(const AvroValue *value);

AvroValue *avro_value_enum_new(int value_index, AvroStr value_repr);

AvroByteArray avro_value_fixed_get(const AvroValue *value);

AvroValue *avro_value_fixed_new(uintptr_t len, AvroByteArray b);

float avro_value_float_get(const AvroValue *value);

AvroValue *avro_value_float_new(float x);

void avro_value_free(AvroValue *v);

int avro_value_int_get(const AvroValue *value);

AvroValue *avro_value_int_new(int n);

long avro_value_long_get(const AvroValue *value);

AvroValue *avro_value_long_new(long n);

AvroValue *avro_value_map_new(uintptr_t capacity);

void avro_value_null_get(const AvroValue *value);

AvroValue *avro_value_null_new(void);

const AvroValue *avro_value_record_get(const AvroValue *record, const AvroStr *field);

AvroStr avro_value_string_get(const AvroValue *value);

AvroValue *avro_value_string_new(AvroStr s);

AvroValue *avro_value_union_new(AvroValue *value);

/*
 * Append a pickled avro value to an avro writer. Writing is not necessarily happening here.
 * Call `avro_writer_flush` to force an actual write.
 */
uintptr_t avro_writer_append(AvroWriter *writer, const AvroByteArray *value);

uintptr_t avro_writer_append2(AvroWriter *writer, AvroValue *value);

/*
 * Flush an avro writer.
 */
uintptr_t avro_writer_flush(AvroWriter *writer);

/*
 * Consume an avro writer and return the avro serialized data.
 */
AvroByteArray avro_writer_into_data(AvroWriter *writer);

/*
 * Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
 */
AvroWriter *avro_writer_new(const AvroSchema *schema, AvroCodec codec);

#endif /* AVRO_RS_BINDINGS_H */
