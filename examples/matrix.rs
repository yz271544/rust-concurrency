use anyhow::Result;
use concurrency::Matrix;
fn main() -> Result<()> {
    let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, [1, 2, 3, 4, 5, 6]);
    println!("a * b: {}", a * b);
    // Output: a * b: {{22 28}, {49 64}}
    Ok(())
}
