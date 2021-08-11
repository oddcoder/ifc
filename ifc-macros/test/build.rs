use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

enum State {
    Pass,
    Fail,
}

lazy_static! {
    static ref TESTS: Mutex<Vec<(State, &'static str)>> = Mutex::new(vec![
        (State::Pass, "test/macro_unsafe.rs"),
        (State::Fail, "test/correct_error_span_let.rs"),
        (State::Pass, "test/macro_unsafe.rs"),
        (State::Pass, "test/macros_pass.rs"),
        (State::Pass, "test/assign_eq_declassify_to_low.rs"),
        (State::Pass, "test/assign_eq_declassify_to_none.rs"),
        (State::Pass, "test/assign_declassify_to_low.rs"),
        (State::Pass, "test/assign_declassify_to_none.rs"),
        (State::Pass, "test/let_declassify_to_low.rs"),
        (State::Pass, "test/match_high_high.rs"),
        (State::Pass, "test/match_low_high.rs"),
        (State::Pass, "test/match_low_low.rs"),
        (State::Pass, "test/match_low_none.rs"),
        (State::Pass, "test/match_none_high.rs"),
        (State::Pass, "test/match_none_low.rs"),
        (State::Pass, "test/match_none_none.rs"),
        (State::Pass, "test/while_high3.rs"),
        (State::Pass, "test/while_none.rs"),
        (State::Pass, "test/while_low.rs"),
        (State::Pass, "test/if_else_high_guard.rs"),
        (State::Pass, "test/if_else_none_guard.rs"),
        (State::Pass, "test/if_else_low_guard.rs"),
        (State::Pass, "test/nested_block_type_inference.rs"),
        (State::Pass, "test/parens.rs"),
        (State::Pass, "test/simple_block.rs"),
        (State::Pass, "test/block_full_scope_test.rs"),
        (State::Pass, "test/binary_high_high.rs"),
        (State::Pass, "test/binary_high_low.rs"),
        (State::Pass, "test/binary_high_none.rs"),
        (State::Pass, "test/binary_low_high.rs"),
        (State::Pass, "test/binary_low_low.rs"),
        (State::Pass, "test/binary_low_none.rs"),
        (State::Pass, "test/binary_none_high.rs"),
        (State::Pass, "test/binary_none_low.rs"),
        (State::Pass, "test/binary_none_none.rs"),
        (State::Pass, "test/format_lowvars.rs"),
        (State::Pass, "test/define_high_as_low.rs"),
        (State::Pass, "test/assign_untyped_to_untyped.rs"),
        (State::Pass, "test/assign_low_to_untyped.rs"),
        (State::Pass, "test/assign_untyped_to_low.rs"),
        (State::Pass, "test/assign_low_to_low.rs"),
        (State::Pass, "test/assign_untyped_to_high.rs"),
        (State::Pass, "test/assign_low_to_high.rs"),
        (State::Pass, "test/assign_high_to_high.rs"),
        (State::Pass, "test/pass_low_to_fn.rs"),
        (State::Pass, "test/pass_high_unsafe_to_fn.rs"),
        (State::Pass, "test/unary.rs"),
        (State::Pass, "test/assignop_untyped_to_untyped.rs"),
        (State::Pass, "test/assignop_low_to_untyped.rs"),
        (State::Pass, "test/assignop_untyped_to_high.rs"),
        (State::Pass, "test/assignop_low_to_high.rs"),
        (State::Pass, "test/assignop_high_to_high.rs"),
        (State::Pass, "test/assignop_untyped_to_low.rs"),
        (State::Pass, "test/assignop_low_to_low.rs"),
        (State::Fail, "test/macros_fail.rs"),
        (State::Fail, "test/useless_declassify.rs"),
        (State::Fail, "test/match_high_low.rs"),
        (State::Fail, "test/match_high_none.rs"),
        (State::Fail, "test/while_high2.rs"),
        (State::Fail, "test/while_high1.rs"),
        (State::Fail, "test/if_else_high_guard4.rs"),
        (State::Fail, "test/if_else_high_guard3.rs"),
        (State::Fail, "test/if_else_high_guard2.rs"),
        (State::Fail, "test/nested_block_type_inference2.rs"),
        (State::Fail, "test/block_access_upper_scope.rs"),
        (State::Fail, "test/variables.rs"),
        (State::Fail, "test/format_highvars.rs"),
        (State::Fail, "test/define_low_as_high.rs"),
        (State::Fail, "test/pass_high_to_fn.rs"),
        (State::Fail, "test/pass_high_mut_to_fn.rs"),
        (State::Fail, "test/pass_high_ref_to_fn.rs"),
        (State::Fail, "test/unused_attribute.rs"),
        (State::Fail, "test/assign_high_to_untyped.rs"),
        (State::Fail, "test/assign_high_to_low.rs"),
        (State::Fail, "test/assignop_high_to_untyped.rs"),
        (State::Fail, "test/assignop_high_to_low.rs"),
    ]);
}

fn worker() {
    let t = trybuild::TestCases::new();
    loop {
        let (state, path) = match TESTS.lock().unwrap().pop() {
            Some(test) => test,
            None => break,
        };
        match state {
            State::Pass => t.pass(path),
            State::Fail => t.compile_fail(path),
        }
    }
}

#[test]
fn tests() {
    let cores = num_cpus::get();
    let mut threads = Vec::with_capacity(cores);
    for _ in 0..cores {
        threads.push(thread::spawn(move || {
            worker();
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
