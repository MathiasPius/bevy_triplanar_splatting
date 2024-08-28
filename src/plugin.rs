use crate::triplanar_material::TriplanarMaterial;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;

const TRIPLANAR_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2631398565563939187);
const BIPLANAR_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1945949403120376729);
const VERTEX_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(3087776660258932868);

pub struct TriplanarMaterialPlugin;

impl Plugin for TriplanarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default());

        load_internal_asset!(
            app,
            TRIPLANAR_SHADER_HANDLE,
            "shaders/triplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            BIPLANAR_SHADER_HANDLE,
            "shaders/biplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VERTEX_SHADER_HANDLE,
            "shaders/vertex.wgsl",
            Shader::from_wgsl
        );
    }
}
