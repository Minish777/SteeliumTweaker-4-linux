pub struct SatHandler;

impl SatHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status(&self) -> String {
        "Система в норме".to_string()
    }
}