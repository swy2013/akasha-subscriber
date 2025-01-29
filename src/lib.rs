mod default_config;
mod dns;
mod r#macro;
mod model;
mod r#type;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use default_config::CLASH;
use model::proxy::{TryIntoClashStyleProxies as _, UrlStyleProxies};
use model::user::User;
use model::{Redirect, SubscriptionUserinfo};
use r#type::Result;
use serde_yaml_ok::{self as yaml, Mapping, Value};
use std::collections::HashMap;
use std::env;
use worker::{event, Context, Env, Request, Response};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let mut config: Value =
        yaml::from_str::<Mapping>(env!["CONFIG", "Must provide env var `CONFIG`"])?.into();
    config.apply_merge()?;
    let config = config
        .as_mapping_mut()
        .expect("Env var `CONFIG` must be a mapping");

    let mut url = req.url()?;

    let users: Vec<User> = yaml::from_value(config.remove("users").unwrap_or_else(|| {
        config
            .remove("default-path")
            .expect("Must provide `default-path` or `users`")
    }))?;
    for user in users {
        if url.path() != user.path() {
            continue;
        }

        let query: HashMap<_, _> = url.query_pairs().collect();

        let mut proxies: Vec<Mapping> =
            yaml::from_value(config.remove("proxies").unwrap_or_default())?;
        if config
            .remove("show-power-by-akasha-subscriber")
            .and_then(|flag| flag.as_bool())
            .is_none_or(|flag| flag)
        {
            proxies.push(yaml::from_str(
                r#"name: "此订阅由github.com/Buer-Nahida/akasha-subscriber虚空订阅器生成"
server: nahida.im
type: hysteria2
ports: 443"#,
            )?);
        }
        let proxies = proxies.try_into_clash_style_proxies(&user, &query).await?;

        let mut res = Response::ok(if query.contains_key("clash") {
            let mut clash_config: Mapping = match query.contains_key("only-proxies") {
                true => Mapping::new(),
                false => yaml::from_value(
                    config
                        .remove("clash")
                        .unwrap_or_else(|| yaml::from_str(CLASH).unwrap()),
                )?,
            };
            clash_config.insert("proxies".into(), proxies.into());
            yaml::to_string(&clash_config)?
        } else {
            let proxies: UrlStyleProxies = proxies.try_into()?;
            BASE64_STANDARD.encode(proxies.into_string())
        })?;

        if let Some(SubscriptionUserinfo {
            upload,
            download,
            total,
            expire,
        }) = config
            .remove("subscription-userinfo")
            .and_then(|v| yaml::from_value(v).ok())
        {
            res.headers_mut().append(
                "subscription-userinfo",
                &format!("upload={upload}; download={download}; total={total}; expire={expire}",),
            )?;
        }

        return Ok(res);
    }

    let Redirect {
        domain,
        status_code,
    } = config
        .remove("redirect")
        .and_then(|v| yaml::from_value(v).ok())
        .unwrap_or_else(|| Redirect::new("nahida.im".into(), 301));
    url.set_host(domain.as_str().into())?;
    Ok(Response::redirect_with_status(url, status_code)?)
}
