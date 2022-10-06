use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Resultat<'a, T> {
    code: i32,
    msg: &'a str,
    data: T,
}

impl<'a, T> Resultat<'a, T> {
    pub fn new(code: i32, msg: &'a str, data: T) -> Self {
        Self { code, msg, data }
    }

    pub fn suc(data: T) -> Self {
        Self::new(0, "success", data)
    }
}

impl<'a> Resultat<'a, ()> {
    pub fn fail(code: i32, msg: &'a str) -> Self {
        Self::new(code, msg, ())
    }

    #[allow(unused)]
    pub fn fail_json(code: i32, msg: &'a str) -> String {
        let this = Self::fail(code, msg);
        this.to_json()
    }

    pub fn fail_json_default(msg: &'a str) -> String {
        let this = Self::fail(-1, msg);
        this.to_json()
    }

    pub fn suc_json_default() -> String {
        let this = Self::suc(());
        this.to_json()
    }
}

impl<'a, T: Serialize> Resultat<'a, T> {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(self);
        json.unwrap_or_default()
    }

    pub fn suc_json(data: T) -> String {
        let this = Self::suc(data);
        this.to_json()
    }
}
