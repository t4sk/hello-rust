fn main() {
    {
        let mut u: u32 = u32::MAX;
        u += 1;

        // Overflow doesn't panic when compiled with --release
        println!("{}", u);
    }

    let u: u32 = u32::MAX;
    // Return None on overflow
    println!("{:?}", u32::checked_add(u, 1));

    // Explicitly allow overflow
    println!("{}", u32::wrapping_add(u, 1));
}
