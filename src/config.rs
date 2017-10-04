use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::sync::RwLock;
use toml;
use xdg::BaseDirectories;
use rand::{thread_rng,Rng};
use apply::Apply;
use regex::Regex;
use serde::{Deserializer,Deserialize};

lazy_static! {
    static ref CONFIG: Config = || -> Result<Config,Box<Error>> {
        let xdg_dirs = BaseDirectories::with_prefix("gpatch")?;
        let config_path = xdg_dirs.place_config_file("config.toml")?;
        let mut file = File::open(config_path)?;
        let mut config = String::new();

        file.read_to_string(&mut config)?;

        let config = toml::from_str(&config);

        log!("config: {:?}", config);

        Ok(config?)
    }()
    .unwrap_or_else(|_| Config::default());

    static ref APP_INDEX: RwLock<Option<usize>> = RwLock::new(None);

    static ref DEFAULT_APP: App = App::default();
}

#[derive(Deserialize,Default,Debug)]
#[serde(default)]
pub struct Config {
    app: Vec<App>,
}

#[derive(Deserialize,Default,Debug)]
#[serde(default)]
pub struct App {
    #[serde(deserialize_with = "deserialize_regex")]
    command: Option<Regex>,
    title: Title,
    hidden_menu_items: Vec<String>,
    supressed_dialogs: Vec<String>,
}

#[derive(Deserialize,Default,Debug)]
struct Title {
    #[serde(deserialize_with = "deserialize_regex")]
    pattern: Option<Regex>,
    replacement: Vec<String>,
}

pub fn init(command: &str) {
    let index = CONFIG.app.iter().position(|app|
        app.command.as_ref()
            .map(|pattern| pattern.is_match(command))
            .unwrap_or(false)
    );

    *APP_INDEX.write().unwrap() = index;
}

fn app() -> &'static App {
    if let Some(index) = *APP_INDEX.read().unwrap() {
        &CONFIG.app[index]
    } else {
        &DEFAULT_APP
    }
}

pub fn is_hidden_menu_item(item: &str) -> bool {
    app().hidden_menu_items.iter().map(String::as_str).any(|other| other == item)
}

pub fn is_supressed_dialog(text: &str) -> bool {
    app().supressed_dialogs.iter().map(String::as_str).any(|needle| text.contains(needle))
}

pub fn title_pattern() -> Option<&'static Regex> {
    app().title.pattern.as_ref()
}

pub fn title_replacement() -> &'static str {
    app().title.replacement.as_slice()
    .apply(|reps| thread_rng().choose(reps))
    .map(String::as_str)
    .unwrap_or("")
}

fn deserialize_regex<'de, D>(de: D) -> Result<Option<Regex>, D::Error> where D: Deserializer<'de> {
    String::deserialize(de)
    .map(|re| Regex::new(&re).ok())
}
