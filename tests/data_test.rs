use wolf_project_210::data::{read_denning_csv, read_reproductive_csv};


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_read_denning_csv() {
        let denning_data = mock_denning_data();
        let reproductive_data = mock_reproductive_data();

    }

    #[test]
    fn test_read_reproductive_csv() {
        let reproductive_data = read_reproductive_csv("data/Wolf_ReproductiveSuccess_AK_CA.csv");
        assert!(reproductive_data.is_ok(), "Failed to read reproductive CSV");
        let data = reproductive_data.unwrap();
        assert!(!data.is_empty(), "Reproductive data should not be empty");
    }
}

