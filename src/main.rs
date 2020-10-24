fn main() {
    println!("{}", esbat::daily_lunar_phase(chrono::Utc::now().date()).as_emoji());
}
