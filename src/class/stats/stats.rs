mod Stats {
    pub struct Stats {
        pub hp: u16,
        pub atk: u16,
        pub def: u16,
        pub sp_atk: u16,
        pub sp_def: u16,
        pub spd: u16,
    }

    impl Stats {
        pub fn get_stats_vector(&self) -> Vec<u16> {
            vec![self.hp, self.atk, self.def, self.sp_atk, self.sp_def, self.spd]
        }
    }

    impl IVs(Stats) {
        pub fn new(stats: Stats) -> Stats {
            for stat in stats.get_stats_vector() {
                if !(0 <= stat <= 31) {
                    panic!("IVs must be between 0 and 31");
                }
            }
            return stats;
        }
    }

    impl EVs(Stats) {
        pub fn new(stats: Stats) -> Stats {
            let total: int = 0;
            for stat in stats.get_stats_vector() {
                total += stat;
                if !(0 <= stat <= 252) {
                    panic!("EVs must be between 0 and 252");
                } else if total > 510 {
                    panic!("Total EVs must be less than or equal to 510");
                }
            }
            return stats;
        }
    }
}