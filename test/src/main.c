#include "tests/tests.h"

int main(void) {
  TEST(my_test, {
    ASSERT(100 == 100);
  });

  TEST(my_second_test, { ASSERT(100 == 100); });

  TEST(my_third_test, { ASSERT(10 == 10); });
}