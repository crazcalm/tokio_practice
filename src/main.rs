async fn add(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    use super::add;

    #[tokio::test]
    async fn test_add() {
        assert_eq!(add(1, 1).await, 2);
    }
}
