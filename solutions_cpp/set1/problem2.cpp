#include <iostream>
#include <vector>

using namespace std;

typedef vector<uint8_t> byte_array;
static char hex_char[] = "0123456789abcdef";

byte_array hex_to_bytes(string hex) {
  hex = hex.length() % 2 == 0 ? hex : "0" + hex;
  byte_array bytes;

  for (int i = 0; i < hex.length(); i += 2) {
    bytes.push_back(stoi(hex.substr(i, 2), nullptr, 16));
  }

  return bytes;
}

int main() {
  string hex1 = "1c0111001f010100061a024b53535009181c";
  string hex2 = "686974207468652062756c6c277320657965";

  byte_array byte_array_1 = hex_to_bytes(hex1);
  byte_array byte_array_2 = hex_to_bytes(hex2);

  string res;
  for (int i = 0; i < byte_array_1.size(); i++) {
    uint8_t xord = byte_array_1[i] ^ byte_array_2[i];

    res.push_back(hex_char[(xord >> 4)]);
    res.push_back(hex_char[xord & 0x0F]);
  }

  cout << res;
}