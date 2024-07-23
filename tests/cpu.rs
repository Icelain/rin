mod cpu {

    #[test]
    #[cfg(test)]
    fn test_cpu() {
        use rin_sys::cpu::CpuInfo;
        let c = CpuInfo::new();
        println!("{:?}", c);

        assert_eq!(c.cores > 0, true);
        assert_eq!(c.model_name.as_str() != "", true);
        assert_eq!(c.vendor_id.as_str() != "", true);
        assert_eq!(c.cpu_speed.len() > 0, true);

        for i in c.cpu_speed {

            let (thread, speed) = i;
            assert_eq!(thread >= 0, true);
            assert_eq!(speed > 0.0, true);

        }
    }
}
