// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
    /* By default, all functions declared within a module are private to other modules.
     * To make a function usable in another module, declare it pub. */
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
