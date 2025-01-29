use super::ClashStyleProxies;
use serde_yaml_ok::Value;

impl From<ClashStyleProxies> for Value {
    fn from(val: ClashStyleProxies) -> Self {
        val.0.into()
    }
}
