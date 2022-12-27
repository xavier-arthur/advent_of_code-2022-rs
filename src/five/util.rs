#[derive(Debug)]
pub struct Directive {
    pub from: usize,
    pub to: usize,
    pub quantity: u32

}

impl Directive {
    pub fn new(from: usize, to: usize, quantity: u32) -> Self {
        Self {
            quantity,
            from, 
            to
        }
    }
}