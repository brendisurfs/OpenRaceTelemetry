#include <unity.h>

#include "imu_math.h"
#include "unity_internals.h"

void setUp(void) {}
void tearDown(void) {}

void test_converts_temp() {
  float expected = 29.33;
  float actual = convert_temp(-2448);

  UNITY_TEST_ASSERT_EQUAL_FLOAT(expected, actual, 14, "Temp should be equal");
}

int main(int argc, char** argv) {
  UNITY_BEGIN();
  test_converts_temp();
  UNITY_END();
}
