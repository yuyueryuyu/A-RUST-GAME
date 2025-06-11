use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::Lit;
use quote::quote;
use syn::ItemFn;

/// 属性宏，用于添加动画状态机进入状态触发器
#[proc_macro_attribute]
pub fn enter(args: TokenStream, input: TokenStream) -> TokenStream {
    let state_name = parse_macro_input!(args as Lit);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let state_str = match state_name {
        Lit::Str(s) => s.value(),
        _ => panic!("Expected string literal for state name"),
    };
    
    let event_name = syn::Ident::new(&format!("__{}EnterEvent", state_str), proc_macro2::Span::call_site());
    let handler_name = syn::Ident::new(&format!("__{}_enter_handler", state_str), proc_macro2::Span::call_site());
    
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    let expanded = quote! {
        #[derive(Event, Debug, Clone)]
        pub struct #event_name {
            pub entity: Entity,
        }
        
        pub fn #handler_name(mut commands: &mut Commands, entity: Entity) {
            commands.trigger(#event_name {entity: entity});
        }
        #(#fn_attrs)*
        pub fn #fn_name(trigger: Trigger<#event_name>, #fn_inputs) #fn_block
    };
    
    TokenStream::from(expanded)
}

/// 属性宏，用于添加动画状态机退出状态触发器
#[proc_macro_attribute]
pub fn exit(args: TokenStream, input: TokenStream) -> TokenStream {
    let state_name = parse_macro_input!(args as Lit);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let state_str = match state_name {
        Lit::Str(s) => s.value(),
        _ => panic!("Expected string literal for state name"),
    };
    let event_name = syn::Ident::new(&format!("__{}ExitEvent", state_str), proc_macro2::Span::call_site());
    let handler_name = syn::Ident::new(&format!("__{}_exit_handler", state_str), proc_macro2::Span::call_site());
    
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    
    let expanded = quote! {
        #[derive(Event, Debug, Clone)]
        pub struct #event_name {
            pub entity: Entity,
        }
        
        pub fn #handler_name(mut commands: &mut Commands, entity: Entity) {
            commands.trigger(#event_name {entity: entity});
        }
        #(#fn_attrs)*
        pub fn #fn_name(trigger: Trigger<#event_name>, #fn_inputs) #fn_block
    };
    
    TokenStream::from(expanded)
}