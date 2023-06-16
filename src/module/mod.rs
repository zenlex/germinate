#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    version: String,
    dev: bool,
    then: Option<Vec<String>>,
}
