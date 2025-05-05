use crate::spec::Provider;
use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

#[cfg(debug_assertions)]
use dotenv::dotenv;

lazy_static! {
    pub static ref PROVIDERS: Vec<Provider> = {
        let json_data = include_bytes!("./providers.json");
        #[allow(clippy::expect_used)]
        let mut providers: Vec<Provider> =
            serde_json::from_slice(json_data).expect("failed to read providers from the JSON file");


        #[cfg(debug_assertions)]
        dotenv().ok();
        // Base for stylesheets
        #[cfg(debug_assertions)]
        let base = format!(
            "http://{}:{}",
            std::env::var("HOST").unwrap_or_default(),
            std::env::var("PORT").unwrap_or_default()
        );

        #[cfg(not(debug_assertions))]
        let base = "https://discovery.storiny.com".to_string();

        // Convert schemas to regex matchers for each provider.
        providers.iter_mut().for_each(|provider| {
            let regex_schemas: Vec<Regex> = provider
                .schemas
                .iter()
                .map(|schema| {
                    let regex_pattern = schema
                        .replace('.', "\\.")
                        .replace('*', "(.+)")
                        .replace('?', "\\?");
                    Regex::new(&regex_pattern)
                        .unwrap_or_else(|_| panic!("invalid regex pattern: {}", schema))
                })
                .collect();

            provider.matchers = regex_schemas;

            // Set base for stylesheets.
            for stylesheet in provider.stylesheets.iter_mut() {
                if Url::parse(stylesheet).is_err() {
                    // Add base to relative URL.
                    *stylesheet = format!("{base}/vendor/stylesheets/{stylesheet}",)
                }
            }
        });

        providers
    };
}
