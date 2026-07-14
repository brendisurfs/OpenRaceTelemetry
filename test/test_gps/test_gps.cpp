#include <unity.h>

#include <cstring>

#include "gps.h"
#include "unity_internals.h"

void setUp(void) {}
void tearDown(void) {}

void test_parses_talker_and_message_type() {
  const char* sentence = "$GPGGA,123519,4807.038,N,01131.000,E*6A";

  NmeaMessage actual = parse_nmea_message(sentence, strlen(sentence));

  TEST_ASSERT_TRUE(actual.valid);
  TEST_ASSERT_EQUAL_STRING("GP", actual.talker);
  TEST_ASSERT_EQUAL_STRING("GGA", actual.message_type);
}

void test_parses_different_talker_and_message_type() {
  const char* sentence = "$GNRMC,123519,A,4807.038,N,01131.000,E*12";

  NmeaMessage actual = parse_nmea_message(sentence, strlen(sentence));

  TEST_ASSERT_TRUE(actual.valid);
  TEST_ASSERT_EQUAL_STRING("GN", actual.talker);
  TEST_ASSERT_EQUAL_STRING("RMC", actual.message_type);
}

void test_rejects_message_missing_dollar_prefix() {
  const char* sentence = "GPGGA,123519,4807.038,N,01131.000,E*6A";

  NmeaMessage actual = parse_nmea_message(sentence, strlen(sentence));

  TEST_ASSERT_FALSE(actual.valid);
}

void test_rejects_message_too_short_to_contain_a_type() {
  const char* sentence = "$GPG";

  NmeaMessage actual = parse_nmea_message(sentence, strlen(sentence));

  TEST_ASSERT_FALSE(actual.valid);
}

void test_rejects_empty_message() {
  const char* sentence = "";

  NmeaMessage actual = parse_nmea_message(sentence, strlen(sentence));

  TEST_ASSERT_FALSE(actual.valid);
}

int main(int argc, char** argv) {
  UNITY_BEGIN();
  RUN_TEST(test_parses_talker_and_message_type);
  RUN_TEST(test_parses_different_talker_and_message_type);
  RUN_TEST(test_rejects_message_missing_dollar_prefix);
  RUN_TEST(test_rejects_message_too_short_to_contain_a_type);
  RUN_TEST(test_rejects_empty_message);
  UNITY_END();
}
