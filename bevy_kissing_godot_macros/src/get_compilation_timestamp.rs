use proc_macro::TokenStream;
use quote::quote;

pub fn get_compilation_timestamp_impl(_: TokenStream) -> TokenStream {
	use std::time::{SystemTime, UNIX_EPOCH};
	let current_system_time = SystemTime::now();
	let duration_since_epoch = current_system_time
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default();
	let current_timestamp = duration_since_epoch.as_secs_f32();

	let result = quote! {
		#current_timestamp
	};

	result.into()
}
