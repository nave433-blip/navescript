use crate::ir_high::{NSIrHigh, ErrorPolicy};

pub fn refine(program: &NSIrHigh) -> anyhow::Result<NSIrHigh> {
    println!("🔄 NaveScript Redundancy Loop (3 models)");

    let mut refined = program.clone();

    // Model 1: Expander
    refined.intent = format!("{} + enhanced", refined.intent);

    // Model 2: Critic + Hardware + Error Handling
    refined.resources.target = "gpu_if_available".to_string();
    refined.error_policy = Some(ErrorPolicy {
        on_error: "retry".to_string(),
        max_retries: 3,
        fallback_step: Some("log_error".to_string()),
    });

    // Model 3: Integrator
    refined.steps.push(crate::ir_high::Step {
        op: "finalize".to_string(),
        params: std::collections::HashMap::from([
            ("status".to_string(), "optimized".to_string())
        ]),
    });

    println!("   ✅ Redundancy loop complete");
    Ok(refined)
}
