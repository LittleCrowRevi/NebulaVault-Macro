extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn impl_component(stream: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(stream as DeriveInput);
    let struct_name = &ast.ident;

    quote!(
        
        impl #struct_name {
            fn free_object(self) {
                let o = Gd::from_object(self);
                o.free();
            }
            
            fn construct() -> Gd<#struct_name> {
                let mut o = #struct_name::alloc_gd();
                let s = Callable::from_object_method(&o, "free_object");
                o.connect("tree_exited".into(), s);
                o
            }
        }
        
        impl Component for #struct_name {
            
        }
    ).into()
}
