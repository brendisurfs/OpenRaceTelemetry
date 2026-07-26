#include "gps.h"

#include <HardwareSerial.h>
#include <TinyGPS++.h>
#include <cstdio>
#include <cstring>
#include "HWCDC.h"
#include "pins_arduino.h"

TinyGPSPlus gps;
HardwareSerial serial_gps(1);

void setup_gps(void) {
  Serial.println("Setting up GPS module");
  serial_gps.begin(9600, SERIAL_6N1, D9, D8);
  Serial.println("setup gps successful");
}

void read_gps_message(void) {
  while (serial_gps.available() > 0) {
    gps.encode(serial_gps.read());
  }

  Serial.print("Quality: ");
  Serial.println(gps.location.FixQuality(), 6);
  Serial.print("Satellites: ");
  Serial.println(gps.satellites.value(), 6);

  if (gps.charsProcessed() > 0) {
    Serial.println("gps processed");
  }

  // if (gps.location.isUpdated()) {
  Serial.print("Lat: ");
  Serial.print(gps.location.lat(), 6);
  Serial.print("Long: ");
  Serial.println(gps.location.lng(), 6);
}
// }

/**
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
