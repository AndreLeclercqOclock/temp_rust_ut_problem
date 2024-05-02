
pub fn public_function() {
    println!("public function");
    private_function();
}

fn private_function() {
    println!("private function");
}

// TODO: I want to move this unit test to the tests.rs file
mod tests {
   #[test]
    fn test_private_function() {
        super::private_function()
    } 
}
