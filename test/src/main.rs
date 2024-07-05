use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_VAR: Mutex<String> = Mutex::new(String::new());
}

fn increment_name(name: &mut String) {
    let mut chars: Vec<char> = name.chars().collect();
    let mut carry = true;

    for i in (0..chars.len()).rev() {
        if carry {
            if chars[i] == 'Z' {
                chars[i] = 'A';
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                carry = false;
            }
        }
    }

    if carry {
        chars.insert(0, 'A');
    }

    name.clear();
    name.extend(chars);
}

fn main() {
    {
        let mut data = GLOBAL_VAR.lock().unwrap();
        *data = String::from("A");

        for _ in 0..(26 * 3) {
            println!("{}", *data);
            increment_name(&mut *data);
        }
    }
}
