error_chain! {
    foreign_links {
        NulError(::std::ffi::NulError);
    }
}
