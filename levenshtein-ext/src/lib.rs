#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::{prelude::*, types::{ZendClassObject, ZendStr}};
use levenshtein::levenshtein;

extern "C" fn request_startup(_ty: i32, _module_number: i32) -> i32 {
    0
}

extern "C" fn request_shutdown(_ty: i32, _module_number: i32) -> i32 {
    0
}

#[php_class]
pub struct Levenshtein {
}

#[php_impl(rename_methods = "none")]
impl Levenshtein {
    pub fn __construct() -> Self {
        Self {}
    }

    pub fn hello_world(#[this] _: &ZendClassObject<Self>) -> String {
        "Hello World".into()
    }

    pub fn distance(#[this] _: &ZendClassObject<Self>, a: String, b: String) -> usize {
        levenshtein(&a, &b)
    } 

}


#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .request_startup_function(request_startup)
        .request_shutdown_function(request_shutdown)
}
