// Adapted from https://github.com/rhaiscript/rhai-fs

use std::fs::File;

#[allow(unused)]
fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    // Make empty file for documentation
    let doc_file_path = std::env::var("OUT_DIR").unwrap() + "/rhai-rand-docs.md";
    let mut doc_file = File::create(doc_file_path).expect("create doc file");

    #[cfg(feature = "metadata")]
    {
        doc_gen::generate_doc(&mut doc_file);
    }
}

#[cfg(feature = "metadata")]
mod doc_gen {
    use rhai::{plugin::*, Engine};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::io::Write;

    // Rhai modules in the `rhai-rand` package.
    mod pkg {
        include!("src/rand.rs");
        include!("src/array.rs");
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[allow(non_snake_case)]
    struct DocFunc {
        pub access: String,
        pub baseHash: u128,
        pub fullHash: u128,
        pub name: String,
        pub namespace: String,
        pub numParams: usize,
        pub params: Option<Vec<HashMap<String, String>>>,
        pub signature: String,
        pub returnType: Option<String>,
        pub docComments: Option<Vec<String>>,
    }

    impl DocFunc {
        pub fn fmt_signature(&self) -> String {
            self.signature
                .replace("Result<", "")
                .replace(", Box<EvalAltResult>>", "")
                .replace("&mut ", "")
                .replace("get$", "")
        }

        pub fn fmt_doc_comments(&self) -> Option<String> {
            self.docComments.clone().map(|dc| {
                dc.join("\n")
                    .replace("/// ", "")
                    .replace("///", "")
                    .replace("/**", "")
                    .replace("**/", "")
                    .replace("**/", "")
            })
        }
    }

    pub fn generate_doc(writer: &mut impl Write) {
        let mut engine = Engine::new();
        let mut fs_module = Module::new();
        combine_with_exported_module!(&mut fs_module, "rhai_lib_path", pkg::rand_functions);
        combine_with_exported_module!(&mut fs_module, "rhai_file_path", pkg::array_functions);
        engine.register_global_module(fs_module.into());

        // Extract metadata
        let json_fns = engine.gen_fn_metadata_to_json(false).unwrap();
        println!("{json_fns}");
        let v: HashMap<String, Vec<DocFunc>> = serde_json::from_str(&json_fns).unwrap();
        for function in v["functions"].clone() {
            println!("{:?}", function);
        }
        let function_list = v["functions"].clone();

        // Write functions
        let mut indented = false;
        for (idx, function) in function_list.iter().enumerate() {
            // Pull out basic info
            let name: &str = &function.name;
            if !name.starts_with("anon") {
                let signature = function.fmt_signature();
                let comments = function.fmt_doc_comments().unwrap_or_default();

                // Check if there are multiple arities, and if so add a header and indent
                if idx < function_list.len() - 1 {
                    if name == function_list[idx + 1].name && !indented {
                        writeln!(writer, "## `{name}`").expect("Cannot write to {doc_file}");
                        indented = true;
                    }
                }

                // Print definition with right level of indentation
                if indented {
                    writeln!(writer, "### `{signature}`\n\n{comments}")
                        .expect("Cannot write to {doc_file}");
                } else {
                    writeln!(writer, "## `{signature}`\n{comments}")
                        .expect("Cannot write to {doc_file}");
                }

                // End indentation when its time
                if idx != 0 && idx < function_list.len() - 1 {
                    if name == function_list[idx - 1].name && name != function_list[idx + 1].name {
                        indented = false;
                    }
                }
            }
        }
    }
}
