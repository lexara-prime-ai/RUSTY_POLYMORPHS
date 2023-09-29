struct Qubit;
struct Bit {
    name: String,
}

impl One for Bit {
    fn is_one(&self) -> bool {
        println!("{} STATE: 1", self.name);
        return true;
    }
}
impl Zero for Bit {
    fn is_zero(&self) -> bool {
        println!("{} STATE: 0", self.name);
        return true;
    }
}
trait One {
    fn is_one(&self) -> bool {
        println!("DEFAULT STATE: 1");
        return true;
    }
}
trait Zero {
    fn is_zero(&self) -> bool {
        println!("DEFAULT STATE: 1");
        return true;
    }
}

trait SuperPositioned: One + Zero {}

impl One for Qubit {
    fn is_one(&self) -> bool {
        println!(
            r#"
            Bit value: 0
        "#
        );
        return true;
    }
}

impl Zero for Qubit {
    fn is_zero(&self) -> bool {
        println!(
            r#"
            Bit value: 1
        "#
        );
        return false;
    }
}

impl SuperPositioned for Qubit {}

#[allow(unused_variables)]
fn is_super_positioned(qubit: &dyn SuperPositioned) -> bool {
    println!(
        r#"
        Super Positioned
            Bit value: 0
            Bit value: 1
        "#
    );
    return true;
}

fn main() {
    let qubit: Qubit = Qubit;
    is_super_positioned(&qubit);

    let bit: Bit = Bit {
        name: "Binary digit".to_string(),
    };
    bit.is_one();
}
