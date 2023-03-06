#[cfg(test)]
mod tests
{
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn public_function() {
    println!("Hello I am a public function");
}

fn private_function() {
    println!("Hello I am a private function");
}

pub fn indirect_access() {
    print!("Accessing a private library using a public function");

    private_function();
}

pub fn new() {}