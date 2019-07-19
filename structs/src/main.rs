struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32
}

fn main() {

    let xiao = User {
        email: String::from("xiao.wang@wework.com"),
        username: String::from("Xiao Wang"),
        active: true,
        sign_in_count: 1,
    };

    println!("User Email is {}", xiao.email);

    let xiaoyu = User {
        email: String::from("xiaoyu.wang@wework.com"),
        username: String::from("Xiaoyu Wang"),
        ..xiao
    };

    println!("{}'s email is {}", xiaoyu.username, xiaoyu.email);

    let tom = build_user("tom.ji@wework.com".to_string(), "Tom Ji".to_string());

    println!("{}'s email is {}", tom.username, tom.email);
}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
