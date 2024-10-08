use graph_craft::wasm_application_io::WasmEditorApi;

pub use graphene_core::text::{bounding_box, load_face, to_path, Font, FontCache};

#[node_macro::node(category(""))]
fn text<'i: 'n>(
	_: (),
	editor: &'i WasmEditorApi,
	text: String,
	font_name: Font,
	#[default(24.)] font_size: f64,
	#[default(1.2)] line_height_ratio: f64,
	#[default(1.)] character_spacing: f64,
) -> crate::vector::VectorData {
	let buzz_face = editor.font_cache.get(&font_name).map(|data| load_face(data));
	crate::vector::VectorData::from_subpaths(to_path(&text, buzz_face, font_size, line_height_ratio, character_spacing, None), false)
}
