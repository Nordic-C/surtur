#pragma once

#include <stdbool.h>

#define BLUE "\033[1;34m"
#define RED "\033[1;31m"
#define WHITE "\033[0m"
#define GREEN "\033[32m"

#define BOLD "\033[1m"
#define STANDARD "\033[0m"

bool canRunTest(const char name[]);