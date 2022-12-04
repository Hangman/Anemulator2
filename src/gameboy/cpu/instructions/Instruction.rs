pub trait Instruction {
    fn run(&self);

    fn get_length() -> u32;

    fn is_prefix() -> bool {
        false
    }
}