use rustpython_vm as vm;
use std::process::ExitCode;

fn run(vm: &vm::VirtualMachine) -> vm::PyResult<()> {
    let scope = vm.new_scope_with_builtins();

    // the file parameter is relative to the directory where the crate's Cargo.toml is located, see $CARGO_MANIFEST_DIR:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    let module = vm::py_compile!(file = "python/main.py");

    let res = vm.run_code_obj(vm.ctx.new_code(module), scope);

    if let Err(exc) = res {
        vm.print_exception(exc);
    }

    Ok(())
}
fn main() -> ExitCode {
    // Add standard library path
    let settings = vm::Settings::default();

    let interp = vm::Interpreter::with_init(settings, |vm| {
	vm.add_frozen(rustpython_pylib::FROZEN_STDLIB);
	let pyby_frozen = rustpython_derive::py_freeze!(dir = "./python", crate_name = "rustpython_compiler_core");
	vm.add_frozen(pyby_frozen);
    });
    let result = interp.enter(run);
    ExitCode::from(interp.run(|_vm| result))
}
