mod cpu {
    
    #[test]
    #[cfg(test)]
    fn test_cpu() {
        
        use rin::cpu::CpuInfo; 
        let mut c = CpuInfo::new();

        c.fetch();

        assert_eq!(c.cores > 0, true);
        assert_eq!(c.model_name.as_str() != "", true);
        assert_eq!(c.vendor_id.as_str() != "", true);
        assert_eq!(c.cpu_mhz > 0., true);

    } 
    

}
