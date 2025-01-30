#include <string>
#include <vector> 
#include <bitset>
#include <iostream>
#include <string>

using namespace std;

typedef vector<uint8_t> byte_array;

string base64_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

byte_array hex_to_bytes(string hex) {
  hex = hex.length() % 2 == 0 ? hex : "0" + hex;
  byte_array bytes;

  for (int i = 0; i < hex.length(); i += 2) {
    bytes.push_back(stoi(hex.substr(i, 2), nullptr, 16));
  }

  return bytes;
}

string base64_encode(byte_array bytes) {
  uint32_t val = 0;
  uint8_t counter = 0;

  string base64_encoded;
  for (int i = 0; i < bytes.size(); i++) {
    val = (val << 8) | bytes[i];
    counter += 8;
    while (counter >= 6) {
      base64_encoded.push_back(base64_table[val >> (counter - 6)]);

      val = val & (1 << (counter - 6)) - 1;
      counter -= 6;
    }
  }

  if (counter > 0)
    base64_encoded.push_back(base64_table[val]);

  return base64_encoded;
}

int main() {
  char hex[] = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

  byte_array bytes = hex_to_bytes(hex);

  string base64_encoded = base64_encode(bytes);

  cout << base64_encoded;
  return 0;
}