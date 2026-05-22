#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::{DistributionManager, DistributionMethod};

    #[test]
    fn test_distribution_manager_initialization() {
        let manager = DistributionManager::new();
        assert!(manager.is_ok(), "DistributionManager failed to initialize");
    }
}
