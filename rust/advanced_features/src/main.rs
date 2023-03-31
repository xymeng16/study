mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_functions_and_closoures;
mod macros;

fn main() {
    // unsafe rust
    unsafe_rust::raw_pointer();
    unsafe_rust::test_split();
    unsafe_rust::test_c_abs();
    unsafe_rust::test_global_var();

    advanced_traits::test_point_add();
    advanced_traits::test_same_name();
    advanced_traits::test_point2_outline_print();
    advanced_traits::test_wrapper();

    advanced_types::test_type_aliases();
    advanced_types::test_never();
    advanced_types::test_dyn_size();

    advanced_functions_and_closoures::test_func_ptr();

    macros::test_procedural_macro();
}