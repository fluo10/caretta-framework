mod derive;

use derive::*;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, FieldsNamed, Ident, parse_macro_input};

fn extract_unique_field_ident<'a>(
    fields: &'a FieldsNamed,
    attribute_arg: &'static str,
) -> &'a Ident {
    let mut fields = extract_field_idents(fields, attribute_arg);
    if fields.len() == 1 {
        return fields.pop().unwrap();
    } else {
        panic!("Model must need one {} field attribute", attribute_arg);
    };
}

fn extract_field_idents<'a>(
    fields: &'a FieldsNamed,
    attribute_arg: &'static str,
) -> Vec<&'a Ident> {
    fields
        .named
        .iter()
        .filter_map(|field| {
            field.attrs.iter().find_map(|attr| {
                if attr.path().is_ident("syncable") {
                    let args: Expr = attr.parse_args().unwrap();

                    match args {
                        Expr::Tuple(arg_tupple) => arg_tupple.elems.iter().find_map(|arg| {
                            if let Expr::Path(arg_path) = arg {
                                if arg_path.path.is_ident(attribute_arg) {
                                    Some(field.ident.as_ref().unwrap())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }),
                        Expr::Path(arg_path) => {
                            if arg_path.path.is_ident(attribute_arg) {
                                Some(field.ident.as_ref().unwrap())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
        })
        .collect()
}

fn extract_fields(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            _ => panic!("all fields must be named."),
        },
        _ => panic!("struct expected, but got other item."),
    }
}

#[proc_macro_derive(Emptiable)]
pub fn emptiable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let field_idents = extract_idents_and_types_from_data_struct(data);
            let is_empty_iter = field_idents.iter().map(|(ident, type_name)| {
                quote! {
                    <#type_name as Emptiable>::is_empty(&self.#ident)
                }
            });
            let empty_iter = field_idents.iter().map(|(ident, type_name)| {
                quote! {
                    #ident: <#type_name as Emptiable>::empty(),
                }
            });
            quote! {
                impl Emptiable for #type_ident {
                    fn empty() -> Self {
                        Self {
                            #(#empty_iter)*
                        }
                    }
                    fn is_empty(&self) -> bool {
                        #(#is_empty_iter)&&*
                    }
                }
            }
            .into()
        }
        _ => panic!("struct or expected, but got other type."),
    }
}

#[proc_macro_derive(Mergeable)]
pub fn mergeable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let field_idents = extract_idents_and_types_from_data_struct(data);
            let merge_iter = field_idents.iter().map(|(ident, type_name)| {
                quote! {
                    <#type_name as Mergeable>::merge(&mut self.#ident, other.#ident);
                }
            });
            quote! {
                impl Mergeable for #type_ident {
                    fn merge(&mut self, mut other: Self){
                        #(#merge_iter)*
                    }
                }
            }
            .into()
        }
        _ => panic!("struct expected, but got other type."),
    }
}

#[proc_macro_derive(RunnableCommand, attributes(runnable_command))]
pub fn runnable_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let idents =
                extract_idents_and_types_from_data_struct_with_attribute(data, "runnable_command");
            let (field_ident, field_type) = unwrap_vec_or_panic(
                idents,
                "RunnableCommand struct must have one field with runnable attribute",
            );

            quote! {
                impl ::caretta_framework::util::RunnableCommand for #type_ident {
                    fn run(self, app_info: ::caretta_framework::types::AppInfo) {
                        <#field_type as ::caretta_framework::util::RunnableCommand>::run(self.#field_ident, app_info)
                    }
                }
            }
            .into()
        }
        Data::Enum(ref variants) => {
            let quote_vec = extract_idents_and_types_from_enum_struct(&variants);
            let quote_iter = quote_vec.iter().map(|(variant_ident, variant_type)| {
                quote! {
                    Self::#variant_ident(x) => <#variant_type as ::caretta_framework::util::RunnableCommand>::run(x, app_info),
                }
            });
            quote! {
                impl ::caretta_framework::util::RunnableCommand for #type_ident {
                    fn run(self, app_info: ::caretta_framework::types::AppInfo) {
                        match self {
                            #(#quote_iter)*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("struct or enum expected, but got union."),
    }
}

#[proc_macro_derive(Service, attributes(service_context))]
pub fn service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let idents =
                extract_idents_and_types_from_data_struct_with_attribute(data, "service_context");
            let (field_ident, field_type) = unwrap_vec_or_panic(
                idents,
                "Service struct must have one field with service_context attribute",
            );

            quote! {
                #[tool_router]
                impl #type_ident {
                    /// Ping device.
                    ///
                    /// This function is for connectivity test so it's works between non-authorized devices.
                    #[tool(description = "Ping to remote device")]
                    async fn dev_ping(
                        &self,
                        params: ::rmcp::handler::server::wrapper::Parameters<::caretta_framework::mcp::model::DevPingRequest>,
                    ) -> Result<::rmcp::Json<::caretta_framework::mcp::model::DevPingResponse>, ::rmcp::model::ErrorData> {
                        <#field_type as ::caretta_framework::mcp::Api>::dev_ping(self.#field_ident,params.0)
                            .await
                            .map(|x| Json(x))
                            .map_err(Into::<ErrorData>::into)
                    }
                }
            }
            .into()
        }
        _ => panic!("struct expected, but got union or enum."),
    }
}
