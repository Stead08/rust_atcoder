fn main(){
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let mut s = t.trim().to_string();

    while !s.is_empty(){
        let len = s.len();
        if s.ends_with("dream") {
            s.truncate(len -5);
        } else if s.ends_with("dreamer") {
            s.truncate(len -7)
        } else if s.ends_with("erase") {
            s.truncate(len -5)
        } else if s.ends_with("eraser") {
            s.truncate(len -6)
        } else {
            break;
        }
    }

    if s.is_empty() {
        print!("YES")
    } else {
        print!("NO")
    }
}
