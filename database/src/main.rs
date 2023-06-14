use cxx::UniquePtr;
fn main() {
    println!("Hello, world!");
    ffi::hundred();
}

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("client.h");

        type Hundred;

        fn hundred() -> UniquePtr<Hundred>;

    }
}
