use crate::*;

pub type ObjGraphics = amethyst::renderer::GraphicsPrefab<Vec<amethyst::renderer::PosNormTex>>;

external_type_uuid!(amethyst::core::Transform, "f3d49cc2-c77e-4dc9-9e1f-c01e9279c999");
external_type_uuid!(amethyst::renderer::CameraPrefab, "15ed1b66-537e-4e75-b52d-cd4659ba53bf");
external_type_uuid!(amethyst::renderer::LightPrefab, "41c40489-269b-4ef5-af7f-675a29473f86");
external_type_uuid!(ObjGraphics, "2f2d6c63-44ff-4e8a-babf-4e5df47519d6");
