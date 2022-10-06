use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Component, Path},
};

use crate::errs::{sresult_from_err, SResult};

/// 获取当前路径的根路径
pub fn root_path() -> Option<String> {
    let buf = std::env::current_dir();
    if buf.is_err() {
        eprintln!("获取当前路径失败, buf={:?}", buf);
        return None;
    }
    let v = buf.unwrap();
    let c = v.as_path().components().collect::<Vec<_>>();

    let mut r = None;
    if cfg!(target_os = "windows") {
        if let Some(Component::Prefix(x)) = c.get(0) {
            r = Some(x.as_os_str().to_os_string().into_string().unwrap());
        }
    } else {
        if let Some(Component::Normal(_)) = c.get(1) {
            if let Some(Component::RootDir) = c.get(0) {
                r = Some(String::from("/"));
            }
        }
    }
    r
}

/// 可追加写入文件
pub fn writable_app_file(path: &str) -> SResult<File> {
    _writable_file(path, true)
}

/// 可覆盖写入文件
pub fn writable_re_file(path: &str) -> SResult<File> {
    _writable_file(path, false)
}

fn _writable_file(path: &str, append: bool) -> SResult<File> {
    let mut open_opt = std::fs::OpenOptions::new();
    open_opt.write(true).append(append);
    let f = open_opt.open(path);
    if f.is_err() {
        let f_err = format!("文件打开失败, f={:?}", f);
        eprintln!("{}", f_err);
        return sresult_from_err(f_err);
    }
    Ok(f.unwrap())
}

/// 读取文件内容
pub fn read_file(path: &str) -> SResult<String> {
    let content = fs::read_to_string(path);
    if content.is_err() {
        return sresult_from_err(content);
    }
    Ok(content.unwrap())
}

/// 读取键值对文件
pub fn read_kv(path: &str) -> SResult<BTreeMap<String, String>> {
    let data = std::fs::read_to_string(&path);
    if data.is_err() {
        return sresult_from_err(data.unwrap_err());
    }
    let data = data.unwrap();
    let mut res = BTreeMap::default();
    for s in data.lines() {
        let auth = s.split_once(":");
        if auth.is_none() {
            continue;
        }
        let (k, v) = auth.unwrap();
        res.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(res)
}

/// 文件大小
/// path：全路径
pub fn file_size<P: AsRef<Path>>(path: P) -> SResult<u64> {
    Ok(File::open(path)?.metadata()?.len())
}

#[test]
fn test() {}
