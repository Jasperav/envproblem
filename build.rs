fn main() {
    println!("cargo:rustc-env=SOME_ENV_VALUE=somevalue");
    dotenv::dotenv().unwrap();
}