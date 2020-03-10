#[macro_export]
macro_rules! jsonrpc {
    (
        $(#[$struct_attr:meta])*
        pub struct $struct_name:ident {$(
            $(#[$attr:meta])*
            pub fn $method:ident(&$selff:ident $(, $arg_name:ident: $arg_ty:ty)*)
                -> $return_ty:ty;
        )*}
    ) => (
        $(#[$struct_attr])*
        pub struct $struct_name {
            pub client: reqwest::blocking::Client,
            pub url: reqwest::Url,
            pub id_generator: $crate::id_generator::IdGenerator,
        }

        impl $struct_name {
            pub fn new(uri: &str, client: reqwest::blocking::Client) -> Self {
                let url = reqwest::Url::parse(uri).expect("uri");
                let id_generator = $crate::id_generator::IdGenerator::new();
                $struct_name { url, id_generator, client, }
            }

            $(
                $(#[$attr])*
                pub fn $method(&$selff $(, $arg_name: $arg_ty)*) -> Result<$return_ty, $crate::error::Error> {
                    let method = String::from(stringify!($method));
                    let params = serialize_parameters!($($arg_name,)*);
                    let id = $selff.id_generator.next();

                    let mut req_json = serde_json::Map::new();
                    req_json.insert("id".to_owned(), serde_json::json!(id));
                    req_json.insert("jsonrpc".to_owned(), serde_json::json!("2.0"));
                    req_json.insert("method".to_owned(), serde_json::json!(method));
                    req_json.insert("params".to_owned(), params);

                    let mut resp = $selff.client.post($selff.url.clone()).json(&req_json).send()?;
                    let output = resp.json::<jsonrpc_core::response::Output>()?;
                    match output {
                        jsonrpc_core::response::Output::Success(success) => {
                            serde_json::from_value(success.result).map_err(Into::into)
                        },
                        jsonrpc_core::response::Output::Failure(failure) => {
                            Err(failure.error.into())
                        }
                    }
                }
            )*
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_parameters {
    () => ( serde_json::Value::Null );
    ($($arg_name:ident,)+) => ( serde_json::to_value(($($arg_name,)+))?)
}
