#[derive(Debug, serde::Serialize)]
pub struct Link {
    pub label: String,
    pub url: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Game {
    pub title: String,
    pub links: Vec<Link>,
}
pub trait PluginInterface {
    //  Add a method to get the schema version of the interface.
    fn version(&self) -> &'static str;

    //  Add a method to get the name of the plugin.
    fn name(&self) -> &'static str;

    //  Add a method to get the author of the plugin.
    fn author(&self) -> &'static str;

    // Add a method to get source of the plugin.
    fn source(&self) -> &'static str;

    //  Add a search method to the interface.
    fn search(&self, query: String) -> Result<Vec<Game>, String>;
}

impl dyn PluginInterface {
    pub fn check_version(&self, version: &str) -> bool {
        version == self.version()
    }
}
