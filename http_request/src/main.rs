fn main() {
    // GET
    let response = ureq::get("https://rust-lang.org").call();

    if response.ok() {
        println!("Reponse body is: {}", response.into_string().unwrap());
    } else {
        println!("Error: {} {}", response.status(), response.status_text());

        // method `into_string()` consumes the response
        let content = response.into_string().unwrap();
        println!("{}", content);
    }

    // POST

    let response = ureq::post("https://httpbin.org/post")
        .set("Authorization", "Token token=secret")
        .set("Content-Language", "zh-CN")
        .send_json(serde_json::json!({
            "name": "Hegwin",
            "skills": ["ruby", "rust"]
        }));

    if response.ok() {
        println!("Reponse body is: {}", response.into_string().unwrap());
    } else {
        println!("Error: {} {}", response.status(), response.status_text());

        // method `into_string()` consumes the response
        let content = response.into_string().unwrap();
        println!("{}", content);
    }
}
