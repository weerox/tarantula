use std::io::Read;
use std::path::{Path, PathBuf};
use syn::{File, ItemMod};
use syn::token::Brace;
use syn::visit_mut::VisitMut;

struct ModuleResolver<P: AsRef<Path>> {
	base: P,
	modules: Vec<String>
}

impl<P: AsRef<Path>> VisitMut for ModuleResolver<P> {
	fn visit_item_mod_mut(&mut self, node: &mut ItemMod) {
		if node.semi.is_some() {
			// TODO Replace this mod by reading the file and insertings its content here in a mod with braces.
			// Make sure that this imported file has already resolved its mods.

			let path = self.base.as_ref().join(self.modules.iter().collect::<PathBuf>());

			let file_path = {
				let module_file = {
					let mut filename = node.ident.to_string();
					filename.push_str(".rs");

					let mut path = path.clone();
					path.push(filename);
					path
				};

				let module_dir = {
					let mut path = path.clone();
					path.push(node.ident.to_string());
					path.push("mod.rs");
					path
				};

				if !module_file.exists() && !module_dir.exists() {
					// FIXME Graceful error handling
					let mut modules = self.modules.clone();
					modules.push(node.ident.to_string());
					
					let module_path = modules.join("::");

					panic!("Couldn't find file for module {:?}", module_path);
				}

				if module_file.exists() {
					module_file
				} else {
					module_dir
				}
			};

			let mut file = if let Ok(file) = std::fs::File::open(&file_path) {
				file
			} else {
				// FIXME Graceful error handling
				panic!("Failed to read file at {:?}", &file_path);
			};

			let mut content = String::new();
			let result = file.read_to_string(&mut content);
			if result.is_err() {
				panic!("Failed to read file at {:?}", &file_path);
			}

			let mut file = if let Ok(file) = syn::parse_file(&content) {
				file
			} else {
				panic!("Failed to parse file at {:?}", &file_path);
			};

			// TODO resolve mods in this new file
			let mut new_module_path = self.modules.clone();
			new_module_path.push(node.ident.to_string());
			resolve_modules(&mut file, self.base.as_ref(), new_module_path);

			node.semi = None;
			node.content = Some((Brace::default(), file.items));
		} else {
			// If this is an inline mod, we add its identifier to the module path and visit its children
			self.modules.push(node.ident.to_string());
			syn::visit_mut::visit_item_mod_mut(self, node);
		}
	}
}

pub fn resolve_modules<P: AsRef<Path>>(file: &mut File, base: P, modules: Vec<String>) {
	ModuleResolver {
		base,
		modules
	}.visit_file_mut(file);
}