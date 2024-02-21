#include <assert.h>
#include <stdio.h>
#include "test.c"

void test();

int main(void) {

  void (*functions[])() = {test};
  functions[0]();
  printf("Hello, World! test\n");
  assert(10 == 100);
}

void test() { printf("Testing\n"); }