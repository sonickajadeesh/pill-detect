fn main() {
    dotenvy::dotenv().ok();
    let key = std::env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set");

    println!("cargo:rustc-env=FIREBASE_API_KEY={key}");
}
