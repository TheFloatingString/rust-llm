use std::vec;

mod llm {
    pub fn ping() -> String {
        let var: String = "ping".to_string();
        return var;
    }

    pub fn dot(a: i32, b: i32) -> i32 {
        return a * b;
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

    #[test]
    fn test_dot_product() {
        assert_eq!(crate::llm::dot(1, 1), 1);
    }
}
