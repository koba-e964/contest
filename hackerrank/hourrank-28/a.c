#include <assert.h>
#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* readline();
char** split_string(char*);

// Complete the lagDuration function below.
int lagDuration(int h1, int m1, int h2, int m2, int k) {
    // Return an integer denoting the duration of time in minutes by which the clock has been lagging.
    return 60 * (h1 + k - h2) + (m1 - m2);
}

int main()
{
    FILE* fptr = fopen(getenv("OUTPUT_PATH"), "w");

    char* q_endptr;
    char* q_str = readline();
    int q = strtol(q_str, &q_endptr, 10);

    if (q_endptr == q_str || *q_endptr != '\0') { exit(EXIT_FAILURE); }

    for (int q_itr = 0; q_itr < q; q_itr++) {
        char** h1M1H2M2 = split_string(readline());

        char* h1_endptr;
        char* h1_str = h1M1H2M2[0];
        int h1 = strtol(h1_str, &h1_endptr, 10);

        if (h1_endptr == h1_str || *h1_endptr != '\0') { exit(EXIT_FAILURE); }

        char* m1_endptr;
        char* m1_str = h1M1H2M2[1];
        int m1 = strtol(m1_str, &m1_endptr, 10);

        if (m1_endptr == m1_str || *m1_endptr != '\0') { exit(EXIT_FAILURE); }

        char* h2_endptr;
        char* h2_str = h1M1H2M2[2];
        int h2 = strtol(h2_str, &h2_endptr, 10);

        if (h2_endptr == h2_str || *h2_endptr != '\0') { exit(EXIT_FAILURE); }

        char* m2_endptr;
        char* m2_str = h1M1H2M2[3];
        int m2 = strtol(m2_str, &m2_endptr, 10);

        if (m2_endptr == m2_str || *m2_endptr != '\0') { exit(EXIT_FAILURE); }

        char* k_endptr;
        char* k_str = readline();
        int k = strtol(k_str, &k_endptr, 10);

        if (k_endptr == k_str || *k_endptr != '\0') { exit(EXIT_FAILURE); }

        int result = lagDuration(h1, m1, h2, m2, k);

        fprintf(fptr, "%d\n", result);
    }

    fclose(fptr);

    return 0;
}

char* readline() {
    size_t alloc_length = 1024;
    size_t data_length = 0;
    char* data = malloc(alloc_length);

    while (true) {
        char* cursor = data + data_length;
        char* line = fgets(cursor, alloc_length - data_length, stdin);

        if (!line) {
            break;
        }

        data_length += strlen(cursor);

        if (data_length < alloc_length - 1 || data[data_length - 1] == '\n') {
            break;
        }

        alloc_length <<= 1;

        data = realloc(data, alloc_length);

        if (!line) {
            break;
        }
    }

    if (data[data_length - 1] == '\n') {
        data[data_length - 1] = '\0';

        data = realloc(data, data_length);
    } else {
        data = realloc(data, data_length + 1);

        data[data_length] = '\0';
    }

    return data;
}

char** split_string(char* str) {
    char** splits = NULL;
    char* token = strtok(str, " ");

    int spaces = 0;

    while (token) {
        splits = realloc(splits, sizeof(char*) * ++spaces);

        if (!splits) {
            return splits;
        }

        splits[spaces - 1] = token;

        token = strtok(NULL, " ");
    }

    return splits;
}
