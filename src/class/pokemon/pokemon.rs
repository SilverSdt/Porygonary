mod Pokemon{
    pub struct Pokemon{
        name: String,
        lvl: u8,
        types: [Type; 2],
        stats: Stats,
        ivs: IVs,
        evs: EVs,
        moves: [Move; 4],
        ability: Ability,
    }

    impl Pokemon {
        pub fn new(name: String, lvl: u8, types: [Type; 2], stats: Stats, ivs: IVs, evs: EVs, moves: [Move; 4], ability: Ability) -> Pokemon {
            if !(1 <= lvl <= 100) {
                panic!("Level must be between 1 and 100");
            }

            return Pokemon {
                name: name,
                lvl: lvl,
                types: types,
                stats: stats,
                ivs: ivs,
                evs: evs,
                moves: moves,
                ability: ability,
            }
        }
    }
}