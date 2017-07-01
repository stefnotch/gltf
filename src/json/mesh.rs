
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use std::collections::HashMap;
use std::fmt;
use json::{accessor, material, Extras, Index};
use validation::Checked;

/// Corresponds to `GL_POINTS`.
pub const POINTS: u32 = 0;

/// Corresponds to `GL_LINES`.
pub const LINES: u32 = 1;

/// Corresponds to `GL_LINE_LOOP`.
pub const LINE_LOOP: u32 = 2;

/// Corresponds to `GL_LINE_STRIP`.
pub const LINE_STRIP: u32 = 3;

/// Corresponds to `GL_TRIANGLES`.
pub const TRIANGLES: u32 = 4;

/// Corresponds to `GL_TRIANGLE_STRIP`.
pub const TRIANGLE_STRIP: u32 = 5;

/// Corresponds to `GL_TRIANGLE_FAN`.
pub const TRIANGLE_FAN: u32 = 6;

/// All valid primitive rendering modes.
pub const VALID_MODES: &'static [u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
];

/// All valid semantic names for Morph targets.
pub const VALID_MORPH_TARGETS: &'static [&'static str] = &[
    "POSITION",
    "NORMAL",
    "TANGENT",
];

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 1,

    /// Corresponds to `GL_LINES`.
    Lines,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip, 

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan,
}

/// Extension specific data for `Mesh`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: HashMap<Checked<Semantic>, Index<accessor::Accessor>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor>>,
    
    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material>>,
    
    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Checked<Mode>,
    
    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub targets: Option<Vec<MorphTargets>>,
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct MorphTargets {
    /// XYZ vertex position displacements of type `[f32; 3]`.
    #[serde(rename = "POSITION")]
    pub positions: Option<Index<accessor::Accessor>>,

    /// XYZ vertex normal displacements of type `[f32; 3]`.
    #[serde(rename = "NORMAL")]
    pub normals: Option<Index<accessor::Accessor>>,

    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    #[serde(rename = "TANGENT")]
    pub tangents: Option<Index<accessor::Accessor>>,
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semantic {
    /// Extra attribute name.
    #[cfg(feature = "extras")]
    Extras(String),

    /// XYZ vertex positions.
    Positions,

    /// XYZ vertex normals.
    Normals,

    /// XYZW vertex tangents where the `w` component is a sign value indicating the
    /// handedness of the tangent basis.
    Tangents,

    /// RGB or RGBA vertex color.
    Colors(u32),

    /// UV texture co-ordinates.
    TexCoords(u32),

    /// Joint indices.
    Joints(u32),

    /// Joint weights.
    Weights(u32),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

impl<'de> de::Deserialize<'de> for Checked<Mode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Mode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MODES)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Mode::*;
                use validation::Checked::*;
                Ok(match value {
                    POINTS => Valid(Points),
                    LINES => Valid(Lines),
                    LINE_LOOP => Valid(LineLoop),
                    LINE_STRIP => Valid(LineStrip),
                    TRIANGLES => Valid(Triangles),
                    TRIANGLE_STRIP => Valid(TriangleStrip),
                    TRIANGLE_FAN => Valid(TriangleFan),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

impl Semantic {
    fn checked(s: &str) -> Checked<Self> {
        use self::Semantic::*;
        use validation::Checked::*;
        match s {
            "NORMAL" => Valid(Normals),
            "POSITION" => Valid(Positions),
            "TANGENT" => Valid(Tangents),
            #[cfg(feature = "extras")]
            _ if s.starts_with("_") => Valid(Extras(s[1..].to_string())),
            _ if s.starts_with("COLOR_") => {
                match s["COLOR_".len()..].parse() {
                    Ok(set) => Valid(Colors(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("TEXCOORD_") => {
                match s["TEXCOORD_".len()..].parse() {
                    Ok(set) => Valid(TexCoords(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("JOINTS_") => {
                match s["JOINTS_".len()..].parse() {
                    Ok(set) => Valid(Joints(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("WEIGHTS_") => {
                match s["WEIGHTS_".len()..].parse() {
                    Ok(set) => Valid(Weights(set)),
                    Err(_) => Invalid,
                }
            },
            _ => Invalid,
        }
    }
}

impl ToString for Semantic {
    fn to_string(&self) -> String {
        use self::Semantic::*;
        match self {
            &Positions => format!("POSITION"),
            &Normals => format!("NORMAL"),
            &Tangents => format!("TANGENT"),
            &Colors(set) => format!("COLOR_{}", set),
            &TexCoords(set) => format!("TEXCOORD_{}", set),
            &Joints(set) => format!("JOINTS_{}", set),
            &Weights(set) => format!("WEIGHTS_{}", set),
            #[cfg(feature = "extras")]
            &Extras(ref name) => format!("_{}", name),
        }
    }
}

impl ToString for Checked<Semantic> {
    fn to_string(&self) -> String {
        match self {
            &Checked::Valid(ref semantic) => semantic.to_string(),
            &Checked::Invalid => format!("<invalid semantic name>"),
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Semantic> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Semantic>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "semantic name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Semantic::checked(value))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
