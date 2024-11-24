#include <stdio.h>
#include <string.h>
#include "magika.h"

int main(int argc, char *argv[]) {
    const ResultWrap_c_void *session_result = magika_session_new();

    if (argc != 2) {
        fprintf(stderr, "Usage: %s <input file>\n", argv[0]);
        return 1;
    }

    printf("input file: %s\n", argv[1]);

    if (session_result == NULL) {
        fprintf(stderr, "session_result is NULL\n");
        return 1;
    }

    if (session_result->error != 0) {
        fprintf(stderr, "error: %lu\n", session_result->error);
        if (session_result->error_message != NULL) {
            fprintf(stderr, "error message: %s\n", session_result->error_message);
        }
        return 1;
    }

    void *session = session_result->pointer;

    const uint8_t *path = (const uint8_t *)argv[1];
    const uintptr_t path_len = strlen((const char *)path);

    const ResultWrap_TypeInfo *identify_result = magika_identify_file_sync(session, path, path_len);

    if (identify_result == NULL) {
        fprintf(stderr, "identify_result is NULL\n");
        return 1;
    }

    if (identify_result->error != 0) {
        fprintf(stderr, "error: %lu\n", identify_result->error);
        if (identify_result->error_message != NULL) {
            fprintf(stderr, "error message: %s\n", identify_result->error_message);
        }
        return 1;
    }

    TypeInfo *info = identify_result->pointer;

    printf("label: %s\n", info->label);

    printf("mime_type: %s\n", info->mime_type);

    printf("group: %s\n", info->group);

    printf("description: %s\n", info->description);

    printf("extensions: %ld\n", info->extensions_length);
    for (int i = 0; i < info->extensions_length; i++) {
        printf("  %s\n", info->extensions[i]);
    }

    printf("is_text: %d\n", info->is_text);

    return 0;
}
