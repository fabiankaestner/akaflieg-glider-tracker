pub fn format_for_display(buf: &[u8]) -> String {
    let mut string_rep = String::new();
    for &byte in buf {
        if byte != 0 {
            let char: Vec<u8> = std::ascii::escape_default(byte).collect();
            string_rep.push_str(std::str::from_utf8(&char).unwrap());
        }
    }
    string_rep
}
