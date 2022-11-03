use std::collections::HashMap;

pub fn url_decoding(hex_string: String) -> String {
    let mut i = 0;
    let mut utf8_map: HashMap<&str, String> = HashMap::new();
    while i < hex_string.len() {
        let c = hex_string.chars().nth(i).unwrap();
        if c == '%' {
            if !utf8_map.contains_key(&hex_string[i..i + 9]) {
                let utf8_bytes: Vec<u8> = Vec::from([
                    hex_to_byte(&hex_string[i + 1..i + 3]),
                    hex_to_byte(&hex_string[i + 4..i + 6]),
                    hex_to_byte(&hex_string[i + 7..i + 9]),
                ]);
                utf8_map.insert(
                    &hex_string[i..i + 9],
                    std::str::from_utf8(&utf8_bytes).unwrap().to_owned(),
                );
            }
            i += 8;
        }
        i += 1;
    }
    let mut utf8_string = hex_string.clone();
    utf8_map.iter().for_each(|(k, v)| {
        utf8_string = utf8_string.replace(k, v);
    });
    utf8_string
}

fn hex_to_byte(hex: &str) -> u8 {
    let mut res: u8 = 0;
    for s in hex.chars() {
        let s = s.to_ascii_lowercase() as u8;
        if s >= b'a' && s <= b'f' {
            if res == 0 {
                res = (s - b'a' + 10) << 4;
            } else {
                res += s - b'a' + 10;
            }
        } else {
            if res == 0 {
                res = s - b'0' << 4;
            } else {
                res += s - b'0';
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::hex::url_decoding;

    #[test]
    fn test_url_coding() {
        let chinese = "小站小记";
        let mut url = String::new();
        for b in chinese.as_bytes() {
            url = url + &format!("%{:02X}", b)
        }
        println!("url:{:?}", url);
        println!("chinese:{:?}", url_decoding(url));
    }
}
