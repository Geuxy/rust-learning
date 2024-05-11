// "pub" means "public".
use std::string::String;

pub struct Company {
    pub name: String,
    pub networth: i64,
}

impl Company {

    pub fn print(&self) {
        println!("Networth of {} is up to ${}", self.name, self.networth);
    }
}

pub fn toyota() -> Company {
    return Company {name: String::from("Toyota"), networth: 294_770_000_000};
}

