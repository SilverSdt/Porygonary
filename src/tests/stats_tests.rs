#[cfg(test)]
mod stats_tests {

    #[test]
    fn test_get_stats_vector() {
        let stats = Stats {
            hp: 100,
            atk: 100,
            def: 100,
            sp_atk: 100,
            sp_def: 100,
            spd: 100,
        };

        assert_eq!(stats.get_stats_vector(), vec![100, 100, 100, 100, 100, 100]);
    }

    #[test]
    fn test_ivs() {
        try{
            let iv = IVs {
                hp: 100,
                atk: 100,
                def: 100,
                sp_atk: 100,
                sp_def: 100,
                spd: 100,
            };
            assert_eq!(True, True)
        } catch {
            assert_eq!(True, False)
        }

    }

    #[test]
    fn test_ivs_panic() {
        try{
            let iv = IVs {
                hp: 100,
                atk: 100,
                def: 100,
                sp_atk: 100,
                sp_def: 100,
                spd: 100,
            };
            assert_eq!(True, False)
        } catch {
            assert_eq!(True, True)
        }

    }
}
