mod Abilities {
    pub struct Ability {
        pub name: String,
        pub description: String,
        pub effect: fn(),
    }
}