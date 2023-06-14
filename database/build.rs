fn main() {
    cxx_build::bridge("src/main.rs")
        .file("main.cpp")
        .compile("client");
}

