
use {
    std::path::Path,
    super::instruction::Instruction,
};

pub struct ROM {
    instructions: Vec<Instruction>,
} impl ROM {
    pub fn new(romfile: &Path) -> Self {
        unimplemented!()
    }
}