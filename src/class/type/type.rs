mod Type {
    pub struct Type {
        pub name: String,
        pub weakness: Vec<String>,
        pub resistance: Vec<String>,
        pub immunities: Vec<String>,
    }

    impl Type {
        pub fn get_multiplier(&self, other: &Type) -> f32 {
            if self.weakness.contains(&other.name) {
                return 2.0;
            } else if self.resistance.contains(&other.name) {
                return 0.5;
            } else if self.immunities.contains(&other.name) {
                return 0.0;
            }
            return 1.0;
        }
    }
}