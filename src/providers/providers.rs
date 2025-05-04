use crate::spec::Provider;
use lazy_static::lazy_static;
use load_dotenv::load_dotenv;
use regex::Regex;
use url::Url;

load_dotenv!();

lazy_static! {
    pub static ref PROVIDERS: Vec<Provider> = {
        let is_dev = option_env!("IS_DEV")
            .map(|val| val == "true")
            .unwrap_or_default();
        let json_data = include_bytes!("./providers.json");
        #[allow(clippy::expect_used)]
        let mut providers: Vec<Provider> =
            serde_json::from_slice(json_data).expect("failed to read providers from the JSON file");

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
                    *stylesheet = format!(
                        "{}/vendor/stylesheets/{stylesheet}",
                        if is_dev {
                            format!("http://{}:{}", env!("HOST"), env!("PORT"))
                        } else {
                            "https://discovery.storiny.com".to_string()
                        }
                    )
                }
            }
        });

        providers
    };
}
