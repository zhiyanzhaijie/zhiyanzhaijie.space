/// URL解码函数
pub fn percent_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex1 = chars.next();
            let hex2 = chars.next();
            if let (Some(h1), Some(h2)) = (hex1, hex2) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    // 只处理ASCII字符，对于UTF-8需要更复杂的处理
                    result.push(byte as char);
                    continue;
                }
            }
            // 如果解码失败，保留原始字符
            result.push('%');
            if let Some(h) = hex1 {
                result.push(h);
            }
            if let Some(h) = hex2 {
                result.push(h);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}
