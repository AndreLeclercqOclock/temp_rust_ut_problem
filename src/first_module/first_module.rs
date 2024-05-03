
pub fn public_function() {
    println!("public function");
    private_function();
}

fn private_function() {
    println!("private function");
}

// Option 1 : Test directly into module file
//mod tests {
//   #[test]
//    fn test_private_function() {
//        super::private_function()
//    } 
//}

// Option 2 : Use submodule ? to test private function.
#[cfg(test)]
#[path ="./tests_modules.rs"]
mod tests_modules;
