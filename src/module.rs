//TODO: create PackageManager trait with parse_deps, install_deps and generate_commands behaviors - might be able to just impl the toml deserializer trait for the ParseDeps part?)
//TODO: Create a struct for each package manager that implements the PackageManager trait and an Enum that holds those structs
// TODO: Create a builder struct that runs the package manager install command for each package manager (CargoBuilder, NpmBuilder, ComposerBuilder, etc)
// TODO: Create a projectBuilder struct that creates the folders, runs the package manager builders, runs the docker builder, db builder, etc.
//TODO: extract npm specific stuff to npm module and add composer and cargo modules
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Module {
    name: String,
    version: String,
    dev: bool,
    then: Option<ThenCommands>,
}

#[allow(unused)]
impl Module {
    pub fn new(name: String, version: String, dev: bool, then: Option<ThenCommands>) -> Self {
        Self {
            name,
            version,
            dev,
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

    pub fn get_then(&self) -> Option<&ThenCommands> {
        self.then.as_ref()
    }
}

type ThenCommands = Vec<Vec<String>>;
