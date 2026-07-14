#include "gps.h"

#include <cstdio>
#include <cstring>

/*
 * NMEA messages are ASCII printable messages with a maximum of 82 chars.
 */
NmeaMessage parse_nmea_message(const char* nmea_message, size_t length) {
  NmeaMessage result{};
  if (length < 6 || nmea_message[0] != '$') {
    result.valid = false;
    return result;
  }

  // copy talker to buffer
  memcpy(result.talker, &nmea_message[1], 2 * sizeof(nmea_message[0]));
  memcpy(result.message_type, &nmea_message[3], 3 * sizeof(nmea_message[0]));
  result.valid = true;

  return result;
}
