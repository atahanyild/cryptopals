const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const HEX_TABLE: &str = "0123456789ABCDEF";

pub fn problem1() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let bytes = hex_to_bytes(hex_str);

    let base64_encoded = bytes_to_base64(bytes);

    println!("{}", base64_encoded);
}

pub fn problem2() {
    let hex_1 = "1c0111001f010100061a024b53535009181c";
    let hex_2 = "686974207468652062756c6c277320657965";

    let byte_array_1 = hex_to_bytes(hex_1);
    let byte_array_2 = hex_to_bytes(hex_2);

    let mut res = String::new();
    for i in 0..byte_array_1.len() {
        let xord = byte_array_1[i] ^ byte_array_2[i];

        res.push(HEX_TABLE.chars().nth((xord >> 4) as usize).unwrap());
        res.push(HEX_TABLE.chars().nth((xord & 0x0F) as usize).unwrap());
    }

    println!("{}", res);
}

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    let formatted_hex_str = if hex_str.len() % 2 == 0 {
        hex_str.to_owned()
    } else {
        format!("0{hex_str}")
    };

    for i in (0..formatted_hex_str.len()).step_by(2) {
        let substr = &formatted_hex_str[i..i + 2];
        let to_int = u8::from_str_radix(substr, 16).unwrap();
        bytes.push(to_int);
    }

    return bytes;
}

fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let mut counter = 0;
    let mut val: u32 = 0;
    let mut base64_string = String::new();

    for byte in bytes {
        val = (val << 8) | byte as u32;
        counter += 8;

        while counter >= 6 {
            let to_encode = val >> (counter - 6);
            base64_string.push(
                BASE64_TABLE
                    .chars()
                    .nth(to_encode.try_into().unwrap())
                    .unwrap(),
            );

            val = val & ((1 << (counter - 6)) - 1);
            counter -= 6;
        }
    }

    if counter > 0 {
        let to_encode = val << (6 - counter);
        base64_string.push(
            BASE64_TABLE
                .chars()
                .nth(to_encode.try_into().unwrap())
                .unwrap(),
        );
    }

    return base64_string;
}
