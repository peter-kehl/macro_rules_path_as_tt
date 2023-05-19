macro_rules! prefixed_lint_versioned {
    ($major_minor:tt, $lint_path:tt) => {};
}

macro_rules! consume_path_only {
    ($lint_path:tt) => {};
}

macro_rules! consume_path_only_as_path_then_pass_down_as_tt {
    ($lint_path:path) => {
        consume_tt_only!($lint_path);
    };
}

macro_rules! consume_tt_only {
    ($lint_path:tt) => {};
}

pub fn test_macro() {
    // The following two fail to compile.
    prefixed_lint_versioned!(1.2, clippy::alloc_instead_of_core);
    consume_path_only!(clippy::alloc_instead_of_core);

    // But, the following does compile!
    consume_path_only_as_path_then_pass_down_as_tt!(clippy::alloc_instead_of_core);
}
