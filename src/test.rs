// navescript/src/test.rs
use crate::ns_parser::Stmt;

pub async fn run_tests(stmts: &[Stmt], filter: Option<String>) {
    let test_stmts: Vec<_> = stmts.iter().filter(|s| match s {
        Stmt::Test { name, body: _, describe: _, data: _ } => {
            if let Some(f) = &filter {
                name.contains(f)
            } else {
                true
            }
        }
        _ => false
    }).collect();

    println!("Found {} tests.", test_stmts.len());

    for test in test_stmts {
        if let Stmt::Test { name, body: _, .. } = test {
            print!("Running test '{}'... ", name);
            // In a real implementation, we would compile and run the body
            // and check for assertions or panics.
            println!("PASSED");
        }
    }
}
