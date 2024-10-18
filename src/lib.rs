mod llm {
    fn ping() {
        println!("ping!")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn ping_test() {
        assert_eq!(0,0);
    }
}