use rocket_okapi::settings::UrlObject;
use rocket_okapi::{rapidoc::*, swagger_ui::*};

pub fn swagger_config() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
    }
}

pub fn rapidoc_config() -> RapiDocConfig {
    RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    }
}
