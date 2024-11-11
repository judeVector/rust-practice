pub mod namehelpers {
    #[allow(dead_code)]
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
    }

    #[allow(dead_code)]
    pub fn database() {
        println!("Hello Fucker");
    }
}

pub mod database {}

pub mod privatefns {
    #[allow(dead_code)]
    pub fn get_age_plus_5(age: u16) -> u16 {
        return age + 5;
    }
}
