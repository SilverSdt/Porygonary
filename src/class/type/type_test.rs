#[cfg(test)]
mod type_tests {
    use super::Type;

    #[test]
    fn test_get_multiplier() {
        let fire = Type {
            name: "Fire",
            weakness: vec!["Water"],
            resistance: vec!["Grass"],
            immunities: vec![],
        };

        let water = Type {
            name: "Water",
            weakness: vec!["Grass"],
            resistance: vec!["Fire"],
            immunities: vec![],
        };

        assert_eq!(fire.get_multiplier(&water), 2.0);
        assert_eq!(water.get_multiplier(&fire), 0.5);
    }
}