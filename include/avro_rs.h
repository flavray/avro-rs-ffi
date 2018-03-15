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

typedef struct AvroSchema AvroSchema;

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

/*
 * Create an avro writer given an avro schema, an avro byte array used as buffer and an avro codec.
 */
AvroWriter *avro_writer_new(const AvroSchema *schema, AvroByteArray *buffer, AvroCodec codec);

#endif /* AVRO_RS_BINDINGS_H */