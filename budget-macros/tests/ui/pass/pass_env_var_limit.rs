//! The `env = "VAR"` form is resolved at macro-expansion time.

#[path = "../support/mock_env.rs"]
mod mock_env;

use budget_macros::budget_cpu_lt;
use mock_env::{budget_panic, Env};

#[budget_cpu_lt(env = "UI_TEST_MAX_CPU")]
fn within_limit() {
    let env = Env::new(999, 0);
}

#[budget_cpu_lt(env = "UI_TEST_MAX_CPU_EXCEEDED")]
fn over_limit() {
    let env = Env::new(1_001, 0);
}

fn main() {
    within_limit();
    let message = budget_panic(over_limit).expect("the budget assertion should have failed");
    assert!(message.contains("CPU instruction cost 1001 exceeded limit 1000"));
}
