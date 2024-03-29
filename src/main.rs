//Using a semicolon after mod front_of_house rather than using a block tells 
//Rust to load the contents of the module from another file with the same name 
//as the module.
mod hello; // module hello defined in hello.rs
mod mylib; // module mylib defined in mylib.rs
mod collection;
mod error;
mod generic;

fn main() {
    hello::log_hello();
    hello::hello();

    mylib::log_mylib();
    mylib::test_back();
    mylib::test_front();

    collection::log_collection();
    collection::vector_test();
    collection::string_test();
    collection::hashmap_test();

    error::log_error();
    error::error_handler();

    generic::log_generic();
    generic::test_trait();
    generic::test_generic();

    return;
}
