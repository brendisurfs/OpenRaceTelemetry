#include "gps.h"

#include <cstdio>
#include <cstring>

/*
 * NMEA messages are ASCII printable messages with a maximum of 82 chars.
 */
NmeaMessage parse_nmea_message(const char* nmea_message, size_t length) {
  char talker[2];
  char message_type[3];

  // copy talker to buffer
  memcpy(talker, &nmea_message[1], 2 * sizeof(nmea_message[0]));

  // copy message type
  memcpy(message_type, &nmea_message[3], 3 * sizeof(nmea_message[0]));

  NmeaMessage result{};
  memcpy(result.talker, talker, sizeof(talker));
  memcpy(result.message_type, message_type, sizeof(message_type));
  result.valid = true;

  return result;
}
