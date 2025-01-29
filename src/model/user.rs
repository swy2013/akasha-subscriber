use serde::{Deserialize, Serialize};
use serde_yaml_ok::{Mapping, Value};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: Option<String>,
    password: Option<String>,
    path: String,
}
impl User {
    pub const fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub const fn password(&self) -> Option<&String> {
        self.password.as_ref()
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
                if let Some(password) = self.password() {
                    proxy.insert("uuid".into(), password.as_str().into());
                }
            }
            "hysteria2" if proxy.get("password").is_none() => {
                if let (Some(password), name) =
                    (self.password(), self.name().unwrap_or(&"user".into()))
                {
                    let password = match proxy.remove("no-user-name") {
                        Some(Value::Bool(true)) => password.into(),
                        _ => format!("{name}:{password}"),
                    };
                    proxy.insert("password".into(), password.into());
                }
            }
            r#type => panic!("Unsupported proxy type: {type}"),
        };
    }
}
