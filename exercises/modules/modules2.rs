// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
    use self::fruits::APPLE as other_fruit;
    pub use self::fruits::PEAR as fruit;
    use self::veggies::CARROT as other_veggie;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub fn get_apple() -> String {
        String::from(self::other_fruit)
    }

    pub fn get_carrot() -> String {
        String::from(self::other_veggie)
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );

    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::get_apple(),
        delicious_snacks::get_carrot()
    );
}
