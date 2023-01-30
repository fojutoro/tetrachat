pub fn get_string() -> String {
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    return name;
}

// TODO: do the hashing
pub fn generate_hash(secret: String) -> String{
    // THIS IS SO STUPID HAHHA
    return secret;
}