mod universe;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut u = universe::Universe::new(64, 64);
        println!("{}", u);
        u.tick();
        print!("{}", u);
    }
}
