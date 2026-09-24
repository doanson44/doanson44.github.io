        assert_eq!(ranking.observation_count(), 1);
        assert_eq!(ranking.ranking_score(), 0);
    }

    #[test]
    fn sparse_observations_keep_a_baseline_for_windowed_returns() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(4_000));
        ranking.observe_at(Some(102.0), Some(8_000));

        assert_eq!(ranking.observation_count(), 3);
        assert!(ranking.return_1s().is_some());
        assert!(ranking.return_3s().is_some());
        assert!(ranking.ranking_score() > 0);
    }

    #[test]
    fn observations_are_bounded_to_five_seconds() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(1_000));
        ranking.observe_at(Some(102.0), Some(6_000));

        assert_eq!(ranking.observation_count(), 3);
        assert!((ranking.return_5s().unwrap() - (102.0 / 101.0 - 1.0)).abs() < 1e-9);
    }

    #[test]
    fn out_of_order_observations_are_ignored() {
        let mut ranking = FuturesTickerRanking::default();