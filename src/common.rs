pub trait SysInfo {

    fn fetch(&mut self);
    fn new() -> Self;

}   
