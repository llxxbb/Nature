extern crate config;

use std::collections::HashMap;

pub fn get_settings() -> HashMap<String, String> {
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();
    // Print out our settings (as a HashMap)
    let content = settings.try_into::<HashMap<String, String>>().unwrap();
    info!("#### The settings.toml information are : \r\n  {:?}", content);
    content
}