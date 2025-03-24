use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitInt};

/// Attribute macro applied to a function to turn it into an MPI unit test.
///
/// Currently, this makes the function an ignored test and creates a wrapper test
/// that runs the function with `mpiexec` using the specified number of processes.
#[proc_macro_attribute]
pub fn mpi_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let processes = parse_macro_input!(attr as LitInt);
    let item = parse_macro_input!(item as ItemFn);
    let fn_name = &item.sig.ident;
    let fn_internal_name = format_ident!("{}_internal", fn_name);
    let fn_wrapper_name = fn_name;
    let body = &item.block;

    // Generate the new functions
    quote! {
        #[test]
        fn #fn_wrapper_name() {
            let module_path = module_path!();
            let test_name = stringify!(#fn_internal_name);

            // Get full test name
            let full_name = if let Some(idx) = module_path.find("::") {
                // Not in the root module, remove the root name and concat test name
                module_path[idx + 2..].to_owned() + "::" + test_name
            } else {
                // In the root module, only use test name
                test_name.to_owned()
            };

            // Run the mpi command
            let mut command = std::process::Command::new("mpiexec");
            command
                .arg("-n")
                .arg(stringify!(#processes))
                .arg("cargo")
                .arg("test")
                .arg(full_name)
                .arg("--")
                .arg("--ignored")
                .arg("--exact");

            let output = command.output().expect("failed to execute command");
            assert!(output.status.success(), "{:?} returned {}\n==== mpiexec stdout: ====\n{}\n==== mpiexec stderr: ====\n{}\n========================", command, output.status, String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());
        }

        #[test]
        #[ignore = "used by mpi_test"]
        fn #fn_internal_name()
            #body
    }
    .into()
}
