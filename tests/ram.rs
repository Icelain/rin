mod ram {

    #[test]
    #[cfg(test)]
    fn test_ram() {

        use rin::ram::RamInfo;

        let mut r = RamInfo::new();
        r.fetch();

        assert_eq!(r.mem_used != 0, true);
        assert_eq!(r.mem_total != 0, true);
        assert_eq!(r.percent_free > 100.0 || r.percent_free < 0.0, false);
        assert_eq!(r.percent_used > 100.0 || r.percent_used < 0.0, false);
        assert_eq!(r.mem_free < r.mem_total, true);

    }

}
