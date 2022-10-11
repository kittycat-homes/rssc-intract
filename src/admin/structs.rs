use dialoguer::Select;

pub struct MenuPage {
    pub items: Vec<MenuItem>,
}

impl MenuPage {
    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let selection = Select::new().items(&self.items).interact()?;
        let fun = self.items[selection].function;
        fun()
    }
}

pub struct MenuItem {
    pub function: fn() -> Result<(), Box<dyn std::error::Error>>,
    pub name: &'static str,
}

impl std::fmt::Display for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
