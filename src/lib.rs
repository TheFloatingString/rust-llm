mod llm {
    pub fn ping() -> String {
        let var: String = "ping".to_string();
        return var;
    }
}

#[cfg(test)]
mod tests {

    #[test]

    fn ping_test() {
        let x: String = crate::llm::ping();
        // println!(crate::llm::ping().to_string());
        assert_eq!(0, 0);
    }
}

