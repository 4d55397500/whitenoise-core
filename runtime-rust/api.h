/* Generated with cbindgen:0.9.1 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int64_t len;
  uint8_t *data;
} ByteBuffer;

ByteBuffer release(const uint8_t *request_ptr, int32_t request_length);
