// 構造体宣言
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct RGB(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    // 構造体をインスタンス化
    // 構造体もミュータブルにすることが可能
    let user1 = User {
        email: String::from("ikeda.web.develop@gmail.com"),
        username: String::from("KentaIkeda"),
        sign_in_count: 1,
        active: true,
    };
    let red = RGB(255, 0, 0);
    println!("{:#?}", red);

    // println!("{:#?}", user1);

    let rebuild_user = build_user(String::from("umauma285@gmail.com"), String::from("Rust!"));
    let user2 = User {
        email: String::from("ikeda.web.develop@gmail.com"),
        username: String::from("TarouTanaka"),
        ..user1
    };

    println!("{:#?}", rebuild_user);
    println!("{:#?}", user2);
}

// 新しいUser structureのインスタンスを返す
fn build_user(email: String, username: String) -> User {
    User {
        // ショートハンド
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}