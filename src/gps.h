#include <cstddef>

// The maximum amount of characters an NMEA message can have
#define MAX_NMEA_MSG 82
/*
 * GPS resource initialization
 */
void setup_gps(void);

// Result of parsing an NMEA sentence's talker/message-type prefix,
// example: "$GPGGA..." where the talker is "GP" and the message_type is "GGA"
typedef struct NmeaMessage {
  char talker[3];
  char message_type[4];
  bool valid;
} nmea_msg_t;

/*
 * Parses the NMEA bytes from the GPS sensor
 *
 */
NmeaMessage parse_nmea_message(const char* nmea_message, size_t length);
