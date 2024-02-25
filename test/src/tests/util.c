#include "util.h"
#include "parser.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

bool canRunTest(const char *name) {
    const char* envVar = getenv("SURTUR_TESTS");
    if (envVar == NULL)
        return false;

    // Check if all tests should be ran
    if (envVar[0] == '*')
        return true;

    size_t valuesCount = 0;
    char** values = parseEnvVar(envVar, &valuesCount);

    for (int i = 0; i < valuesCount; i++) {
        char* value = values[i];
        if (!strcmp(name, value))
            return true;
    }

    return false;
}