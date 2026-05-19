use crate::ir_high::ErrorPolicy;

pub fn apply_error_handling(step: &mut crate::ir_high::Step, policy: &Option<ErrorPolicy>) {
    if let Some(p) = policy {
        step.params.insert("error_policy".to_string(), p.on_error.clone());
        step.params.insert("max_retries".to_string(), p.max_retries.to_string());
    }
}
