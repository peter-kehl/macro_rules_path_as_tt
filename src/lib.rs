macro_rules! prefixed_lint_versioned {
    ($major_minor:tt, $lint_path:tt) => {
    }
}

macro_rules! consume_path_only {
    ($lint_path:tt) => {
    }
}

pub fn test_macro() {
    prefixed_lint_versioned!(1.2, clippy::alloc_instead_of_core);

    consume_path_only!(clippy::alloc_instead_of_core);
}
