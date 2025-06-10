use std::collections::HashMap;

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

/// 解析交互式组件的参数字符串为键值对
pub fn parse_interactive_params(params_str: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    // 移除可能的引号
    let cleaned_str = params_str.trim().trim_matches('"').trim();
    
    if cleaned_str.is_empty() {
        return params;
    }
    
    // 解析格式如 key1="value1" key2="value2" 的字符串
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut in_key = true;
    let mut in_quotes = false;
    let mut escape_next = false;
    
    for c in cleaned_str.chars() {
        if escape_next {
            if in_key {
                current_key.push(c);
            } else {
                current_value.push(c);
            }
            escape_next = false;
            continue;
        }
        
        if c == '\\' {
            escape_next = true;
            continue;
        }
        
        if in_key {
            if c == '=' {
                in_key = false;
                continue;
            }
            if !c.is_whitespace() {
                current_key.push(c);
            } else if !current_key.is_empty() {
                // 如果键后面有空格，等待=号
                continue;
            }
        } else {
            if c == '"' {
                in_quotes = !in_quotes;
                if !in_quotes && !current_key.is_empty() {
                    // 引号结束，添加当前的键值对
                    params.insert(current_key.clone(), current_value.clone());
                    current_key.clear();
                    current_value.clear();
                    in_key = true;
                }
                continue;
            }
            
            if in_quotes {
                current_value.push(c);
            } else if !c.is_whitespace() {
                // 不在引号内的非空白字符，可能是没有引号的值
                current_value.push(c);
            } else if !current_value.is_empty() {
                // 空白字符结束了无引号的值
                params.insert(current_key.clone(), current_value.clone());
                current_key.clear();
                current_value.clear();
                in_key = true;
            }
        }
    }
    
    // 处理最后一个键值对
    if !current_key.is_empty() && !current_value.is_empty() {
        params.insert(current_key, current_value);
    }
    
    params
}