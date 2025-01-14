use crate::ipc::model::TypstRenderResponse;
use crate::project::ProjectManager;
use base64::Engine;
use std::sync::Arc;
use tauri::Runtime;
use typst::geom::Color;

#[tauri::command]
pub async fn typst_render<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    page: usize,
    scale: f32,
) -> Result<TypstRenderResponse, ()> {
    println!("rendering page: {}", page);
    if let Some(project) = project_manager.get_project(&window) {
        let cache = project.cache.read().unwrap();
        if let Some(frame) = cache.document.as_ref().and_then(|doc| doc.pages.get(page)) {
            let bmp = typst::export::render(frame, scale, Color::WHITE);
            if let Ok(image) = bmp.encode_png() {
                println!("render complete for page: {}", page);
                let b64 = base64::engine::general_purpose::STANDARD.encode(image);
                return Ok(TypstRenderResponse {
                    image: b64,
                    width: bmp.width(),
                    height: bmp.height(),
                });
            }
        }
    }
    Err(())
}
