#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    version: String,
    dev: bool,
    features: Option<Vec<String>>,
    then: Option<ThenCommands>,
}

type ThenCommands = Vec<Vec<String>>;

impl Module {
    pub fn new(
        name: String,
        version: String,
        dev: bool,
        then: Option<ThenCommands>,
        features: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            version,
            dev,
            features,
            then,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn is_dev(&self) -> bool {
        self.dev
    }

    pub fn get_features(&self) -> Option<&Vec<String>> {
        self.features.as_ref()
    }

    pub fn get_then(&self) -> Option<&ThenCommands> {
        self.then.as_ref()
    }
}
