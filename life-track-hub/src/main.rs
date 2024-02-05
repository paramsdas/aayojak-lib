use time::OffsetDateTime;

fn main() {
    let now = OffsetDateTime::now_local().unwrap();
    println!("{now}");
}
