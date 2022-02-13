use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        const_impl: {
            all(
                feature = "const_trait_impl",
                feature = "const_default_impls",
                feature = "const_fn_trait_bound"
            )
        },
    }
}
