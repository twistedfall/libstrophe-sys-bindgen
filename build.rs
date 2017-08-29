#[cfg(feature = "buildtime_bindgen")]
fn build_wrapper() {
	extern crate bindgen;

	use std::env;
	use std::path::PathBuf;

	#[derive(Debug)]
	struct PCallbacks;

	impl bindgen::callbacks::ParseCallbacks for PCallbacks {
		fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
			if name == "XMPP_EOK" {
				Some(bindgen::callbacks::IntKind::Custom { name: "::std::os::raw::c_int", is_signed: true })
			} else {
				None
			}
		}
	}

	let builder = bindgen::builder()
		.header("wrapper.h")
		.parse_callbacks(Box::new(PCallbacks))

		.blacklist_type("max_align_t")
		.blacklist_type("wchar_t")

		.rustified_enum("xmpp_log_level_t")
		.rustified_enum("xmpp_conn_type_t")
		.rustified_enum("xmpp_conn_event_t")
		.rustified_enum("xmpp_error_type_t")
	;

	let bindings = builder.generate().expect("Unable to generate bindings");

	// Write the bindings to the src/ffi.rs file.
	let mut out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	out_path.push("src/ffi.rs");
	bindings.write_to_file(&out_path).expect(&format!("Couldn't write bindings to: {}", out_path.display()));
}

fn main() {
	println!("cargo:rustc-link-lib=strophe");
	#[cfg(feature = "buildtime_bindgen")]
	build_wrapper();
}
