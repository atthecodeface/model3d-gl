//a Imports
use model3d_base::ShortIndex;

use crate::{Gl, ShaderMaterialBaseData, TextureId};

//a Material
//tp Material
/// A null material for now
#[derive(Debug)]
pub struct Material {
    // <G: Gl> {
    base_data: ShaderMaterialBaseData,
    textures: [(TextureId, ShortIndex); 8],
}

// impl<G: Gl> Material<G> {
impl Material {
    //cp create
    /// Create a GL material for a given context within the object
    ///
    /// This is invoked when the object is being made instantiable;
    pub fn create<M, G: Gl>(
        _context: &mut G,
        _object: &model3d_base::Object<M, G>,
        material: &M,
    ) -> Result<Self, ()>
    where
        M: model3d_base::Material,
    {
        let base_data = ShaderMaterialBaseData::of_material(material);
        let mut textures = [(TextureId::None, ShortIndex::none()); 8];
        let mut i = 0;
        for aspect in [
            model3d_base::MaterialAspect::Color,
            model3d_base::MaterialAspect::Normal,
            model3d_base::MaterialAspect::MetallicRoughness,
            model3d_base::MaterialAspect::Occlusion,
            model3d_base::MaterialAspect::Emission,
        ] {
            let ti = material.texture(aspect);
            if ti.is_some() {
                textures[i] = (TextureId::of_material_aspect(aspect), ti);
                i += 1;
            };
        }
        eprintln!("Textures {textures:?}");
        Ok(Self {
            base_data,
            textures,
        })
    }
    pub fn base_data(&self) -> &ShaderMaterialBaseData {
        &self.base_data
    }
    pub fn textures(&self) -> &[(TextureId, ShortIndex)] {
        &self.textures
    }
}

//ip Display for Material
// impl<G: Gl> std::fmt::Display for Material<G> {
impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

//ip MaterialClient for Material
// impl<G: Gl> model3d_base::MaterialClient for Material<G> {}
impl model3d_base::MaterialClient for Material {}
