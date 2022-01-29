mod cpu {
    
    #[test]
    #[cfg(test)]
    fn test_cpu() {
        
        use rin_sys::cpu::CpuInfo; 
        let mut c = CpuInfo::new();

        c.fetch();
        
        println!("{:?}", c);

        assert_eq!(c.cores > 0, true);
        assert_eq!(c.model_name.as_str() != "", true);
        assert_eq!(c.vendor_id.as_str() != "", true);
        assert_eq!(c.cpu_speed.replace("GHz", "").parse::<f32>().unwrap() >= 0. , true);

    } 
    

}
