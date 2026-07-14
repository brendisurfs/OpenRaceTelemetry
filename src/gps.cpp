#include "gps.h"

#include <cstring>

/*
 * NMEA messages are ASCII printable messages with a maximum of 82 chars.
 */
NmeaMessage parse_nmea_message(const char* nmea_message, size_t length) {
  char talker[3];
  char message_type[4];

  // copy talker to buffer
  memcpy(talker, &nmea_message[0], 3 * sizeof(nmea_message[0]));

  // copy message type
  memcpy(message_type, &nmea_message[3], 4 * sizeof(nmea_message[0]));

  return {.talker = {*talker}, .message_type = {*message_type}, .valid = true};
}
