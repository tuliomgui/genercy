use tera::Tera;
use std::sync::OnceLock;

pub struct Templates {}

static INSTANCE: OnceLock<Tera> = OnceLock::new();

impl Templates {
    pub fn get_templater() -> &'static Tera {
        INSTANCE.get_or_init(|| {
            let templater = match Tera::new("templates/*.html") {
                Ok(t) => t,
                Err(error) => {
                    panic!("Templater could not be initialized: {error}");
                }
            };
            return templater
        })
    }
}