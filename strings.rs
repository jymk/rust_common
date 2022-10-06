/// String转为静态生命周期str
pub fn str_to_static(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn u8s_to_str(data: &[u8]) -> String {
    let res = data.iter().fold(vec![], |mut x, y| {
        x.push(*y as char);
        x
    });
    println!("u8s_to_chars={:?}", String::from_iter(res.clone()));
    // println!("u8s_to_chars={:?}", res);
    String::from_iter(res)
}

pub fn unescape(s: &str) -> String {
    s.trim_matches('"')
        .replace(r#"\""#, r#"""#)
        .replace(r#"\\""#, r#"\""#)
        .replace(r#"\'"#, "'")
        .replace(r#"\\r"#, r#"\r"#)
        .replace(r#"\\n"#, r#"\n"#)
}

/// 提取出数字和字母，并转小写
pub fn extract_normal_lower_char(s: &str) -> String {
    // let mut res = String::default();
    // for c in s.chars() {
    //     match c {
    //         '0'..='9' | 'a'..='z' | 'A'..='Z' => res.push(c),
    //         _ => {}
    //     }
    // }
    s.to_lowercase()
}

#[test]
fn test() {}
