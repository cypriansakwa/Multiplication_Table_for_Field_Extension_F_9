/// A struct to represent elements of the field \( \mathbb{F}_{3^2} \)
#[derive(Clone, Copy, Debug)]
struct F3x2 {
    a: u8, // coefficient for 1
    b: u8, // coefficient for x
}

impl std::fmt::Display for F3x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),       // No `x` term
            (0, 1) => write!(f, "x"),           // Omit 1 for `x`
            (0, b) => write!(f, "{}x", b),      // Display `bx` for b = 2
            (1, 1) => write!(f, "1 + x"),       // Special case for `1 + x`
            (a, 1) => write!(f, "{} + x", a),   // Omit 1 for `x`
            (a, b) => write!(f, "{} + {}x", a, b), // General case
        }
    }
}

impl F3x2 {
    /// Creates a new field element
    fn new(a: u8, b: u8) -> Self {
        F3x2 { a: a % 3, b: b % 3 }
    }


    /// Multiplies two elements of the field \( \mathbb{F}_{3^2} \)
    fn multiply(self, other: F3x2) -> F3x2 {
        let ac = self.a * other.a;
        let ad_bc = self.a * other.b + self.b * other.a;
        let bd = self.b * other.b;

        // Multiply considering the relation \( x^2 = 2 \)
        F3x2 {
            a: (ac + 2 * bd) % 3,
            b: (ad_bc) % 3,
        }
    }
}

fn main() {
    // Define the 9 elements of \( \mathbb{F}_{3^2} \)
    let field_elements = [
        F3x2::new(0, 0), // 0
        F3x2::new(1, 0), // 1
        F3x2::new(2, 0), // 2
        F3x2::new(0, 1), // x
        F3x2::new(1, 1), // x + 1
        F3x2::new(2, 1), // x + 2
        F3x2::new(0, 2), // 2x
        F3x2::new(1, 2), // 2x + 1
        F3x2::new(2, 2), // 2x + 2
    ];

    // Print the field elements
    println!("Field Elements in \\mathbb{{F}}_{{3^2}}:");
    for (i, elem) in field_elements.iter().enumerate() {
        println!("Element {}: {}", i, elem);
    }

    // Draw the multiplication table
    println!("\nMultiplication Table for \\mathbb{{F}}_{{3^2}}:\n");

    // Print the header
    print!("     ");
    for elem in field_elements.iter() {
        print!("{:>10} ", elem);
    }
    println!();

    // Print each row of the multiplication table
    for elem1 in field_elements.iter() {
        print!("{:>3} |", elem1); // print row label
        for elem2 in field_elements.iter() {
            let product = elem1.multiply(*elem2);
            print!("{:>10} ", product);
        }
        println!();
    }
}

