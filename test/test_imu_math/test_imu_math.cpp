#include <unity.h>

#include <cstdint>

#include "imu_math.h"
#include "unity_internals.h"

void setUp(void) {}
void tearDown(void) {}

void test_converts_temp() {
  float expected = 29.33;
  float actual = convert_temp(-2448);

  UNITY_TEST_ASSERT_EQUAL_FLOAT(expected, actual, 14, "Temp should be equal");
}

void test_calculate_roll_flat() {
  int16_t accel_y = -224;
  int16_t accel_z = 15820;

  float expected = -1.0;
  float actual = calculate_roll(accel_y, accel_z);

  UNITY_TEST_ASSERT_EQUAL_FLOAT(expected, actual, 24,
                                "Actual Degree angle of roll should be 0.0");
}

void test_calc_roll_45_degree_right() {
  int16_t accel_y = 324;
  int16_t accel_z = 12752;

  float expected = 45.0;
  float actual = calculate_roll(accel_y, accel_z);

  UNITY_TEST_ASSERT_FLOAT_IS_NOT_NAN(actual, 49, "Roll should not be NaN");
  // UNITY_TEST_ASSERT_EQUAL_FLOAT(expected, actual, 24,
  //                               "Actual Degree angle of roll should
  //                               be 45.0");
}
void test_pitch_flat() {
  int16_t accel_y = 324;
  int16_t accel_z = 12752;
}

void test_complementary_filter() {
  /*
   * accel: [-13244 636 9300]    gyro: [-1107 1713 -360]
   * accel: [-11708 384 8100]    gyro: [-1550 -151 -564]
   * accel: [-12984 724 8260]    gyro: [-194 265 174]
   * accel: [-13148 604 7188]    gyro: [238 -783 463]
   * accel: [-12824 680 7636]    gyro: [-104 -331 91]
   * accel: [-13772 696 7148]    gyro: [713 -2615 586]
   * accel: [516 1808 13172]     gyro: [2109 -14134 1781]
   * accel: [11144 932 10108]    gyro: [-624 -1837 -1530]
   * accel: [11360 1240 10996]   gyro: [-179 -2145 152]
   * accel: [5156 4988 11796]    gyro: [6962 4098 -2592]
   * accel: [2684 11208 10572]   gyro: [2391 -2708 -142]
   * accel: [3036 12592 9932]    gyro: [40 -877 13]
   * accel: [3656 12488 10532]   gyro: [924 -440 362]
   * accel: [2640 12520 10060]   gyro: [-285 39 -266]
   * accel: [2736 12352 9972]    gyro: [85 -304 -246]
   * accel: [2720 12024 9716]    gyro: [-491 -293 -108]
   * accel: [5492 10624 11876]   gyro: [-31898 18145 2446]
   * accel: [-2372 -9564 17020]  gyro: [-5043 2049 -533]
   * accel: [-3620 -12436 8816]  gyro: [-2119 -191 -161]
   * accel: [-3480 -12752 8308]  gyro: [1745 -687 -199]
   * accel: [-3652 -13088 10600] gyro: [168 -629 -1202]
   * accel: [-2188 -7824 13788]  gyro: [25959 -9492 4421]
   * accel: [528 11220 10020]    gyro: [6510 -1198 -45]
   * accel: [964 12184 8524]     gyro: [679 1066 -369]
   * accel: [-560 8116 9028]     gyro: [571 -3486 -174]
   * accel: [-404 -256 15720]    gyro: [51 -12 -217]
   * accel: [-424 -240 15576]    gyro: [61 54 -216]
   */

  // int16_t accel_x = -13244;
  int16_t accel_y = 636;
  int16_t accel_z = 9300;
  int16_t gyro_x = -1107;
  // int16_t gyro_y = 1713;
  // int16_t gyro_z = -360;

  float roll = calculate_roll(accel_y, accel_z);
  float gyro_rate = gyro_x / 131.0;  // Gyroscope Sensitivity

  float expected = 0.0;
  float actual = complemetary_filter(roll, gyro_rate, 0.01);
  UNITY_TEST_ASSERT_EQUAL_FLOAT(
      expected, actual, 84, "Expected complemetary filter to match expected");
}

int main(int argc, char** argv) {
  UNITY_BEGIN();
  RUN_TEST(test_converts_temp);
  RUN_TEST(test_calculate_roll_flat);
  RUN_TEST(test_complementary_filter);
  UNITY_END();
}
