#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ResultWrap_c_void {
  void *pointer;
  uintptr_t error;
  const uint8_t *error_message;
} ResultWrap_c_void;

typedef struct TypeInfo {
  const uint8_t *label;
  const uint8_t *mime_type;
  const uint8_t *group;
  const uint8_t *description;
  uintptr_t extensions_length;
  const uint8_t *const *extensions;
  uint8_t is_text;
} TypeInfo;

typedef struct ResultWrap_TypeInfo {
  struct TypeInfo *pointer;
  uintptr_t error;
  const uint8_t *error_message;
} ResultWrap_TypeInfo;

const struct ResultWrap_c_void *magika_session_new(void);

void magika_session_free(void *session);

const struct ResultWrap_TypeInfo *magika_identify_file_sync(void *session,
                                                            const uint8_t *path,
                                                            uintptr_t path_len);
