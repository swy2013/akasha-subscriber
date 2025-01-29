use serde::{Deserialize, Serialize};
use serde_yaml_ok::{Mapping, Value};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
    path: String,
}
impl User {
    pub const fn name(&self) -> &String {
        &self.name
    }

    pub const fn password(&self) -> &String {
        &self.password
    }

    pub const fn path(&self) -> &String {
        &self.path
    }

    pub fn insert_password(&self, proxy: &mut Mapping) {
        match proxy
            .get("type")
            .expect("Must be provide `type` of proxy")
            .as_str()
            .expect("`type` of proxy must be a string")
        {
            "vless" | "vmess" if proxy.get("uuid").is_none() => {
                proxy.insert("uuid".into(), self.password().as_str().into());
            }
            "hysteria2" if proxy.get("password").is_none() => {
                let password = match proxy.remove("no-user-name") {
                    Some(Value::Bool(true)) => self.password(),
                    _ => &format!("{}:{}", self.name(), self.password()),
                };
                proxy.insert("password".into(), password.as_str().into());
            }
            r#type => panic!("Unsupported proxy type: {type}"),
        };
    }
}
