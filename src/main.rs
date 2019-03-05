use um::mem::Mem;

fn main() {
    println!("Welcome to the machine.");
    let mut m = Mem::new();
    let a0 = m.alloc(10);
    m.free(a0);
    println!("Allocated, freed and gone.");
}
