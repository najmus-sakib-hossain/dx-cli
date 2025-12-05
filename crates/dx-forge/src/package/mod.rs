#[derive(Debug, Default)]
pub struct PackageManager;

impl PackageManager {
    pub fn install(&self, name: &str) {
        println!("Installing package {name} (stub)");
    }
}
