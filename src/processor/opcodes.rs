pub fn clear_screen() {
    println!("Cleared screen");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_screen_test() {
        clear_screen();
    }
}
