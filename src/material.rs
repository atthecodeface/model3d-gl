//a Imports
// use crate::{Gl, GlMaterial, RenderContext, Renderable};

//a Material
//tp Material
/// A null material for now
#[derive(Debug, Default, Clone)]
pub struct Material();

//ip Display for Material
impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}
//ip MaterialClient for Material
impl model3d_base::MaterialClient for Material {}
// fn create(
// &mut self,
// _material: &dyn model3d_base::Material<R>,
// _render_context: &mut R::Context,
// ) {
//     }
//     fn drop(
//         &mut self,
//         _material: &dyn model3d_base::Material<R>,
//         _render_context: &mut R::Context,
//     ) {
//     }
