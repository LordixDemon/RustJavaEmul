pub(super) const PROJECTION_PARALLEL_SCALE: i32 = 0;
pub(super) const PROJECTION_PERSPECTIVE_FOV: i32 = 1;
pub(super) const PROJECTION_PERSPECTIVE_WH: i32 = 2;
pub(super) const PROJECTION_PARALLEL_SIZE: i32 = 3;

pub(super) const MAT_COLORKEY: i32 = 0x01;
pub(super) const MAT_DOUBLE_FACE: i32 = 0x10;
pub(super) const MAT_BLEND_MASK: i32 = 0x06;
pub(super) const MAT_LIGHTING: i32 = 0x20;
pub(super) const MAT_SPECULAR: i32 = 0x40;
pub(super) const MAT_FLAT_NORMAL: i32 = 0x80;
pub(super) const MAT_ZSORT_NEAR: i32 = 0x100;
pub(super) const MAT_ZSORT_FAR: i32 = 0x200;
pub(super) const MAT_ZSORT_MASK: i32 = MAT_ZSORT_NEAR | MAT_ZSORT_FAR;
pub(super) const MAT_MASK: i32 = MAT_COLORKEY | MAT_BLEND_MASK | MAT_DOUBLE_FACE | MAT_LIGHTING | MAT_SPECULAR | MAT_FLAT_NORMAL | MAT_ZSORT_MASK;

pub(super) const IMPLICIT_COLOR_KEY_MAX_TEXELS: usize = 8192;
pub(super) const IMPLICIT_COLOR_KEY_MIN_OPAQUE_TEXELS: usize = 8;
pub(super) const IMPLICIT_COLOR_KEY_ZERO_RATIO_NUMERATOR: usize = 70;
pub(super) const IMPLICIT_COLOR_KEY_ZERO_RATIO_DENOMINATOR: usize = 100;
pub(super) const IMPLICIT_COLOR_KEY_EDGE_ZERO_RATIO_NUMERATOR: usize = 70;
pub(super) const IMPLICIT_COLOR_KEY_MIN_ZERO_RATIO_NUMERATOR: usize = 50;

pub(super) const TRI_C_STRIDE: usize = 5;
pub(super) const QUAD_C_STRIDE: usize = 6;
pub(super) const TRI_T_STRIDE: usize = 7;
pub(super) const QUAD_T_STRIDE: usize = 9;
pub(super) const PATTERN_STRIDE: usize = 4;
pub(super) const BONE_STRIDE: usize = 15;

pub(super) const ACTION_TABLE_CLASS: &str = "com/mascotcapsule/micro3d/v3/ActionTable";
pub(super) const AFFINE_TRANS_CLASS: &str = "com/mascotcapsule/micro3d/v3/AffineTrans";
pub(super) const EFFECT_3D_CLASS: &str = "com/mascotcapsule/micro3d/v3/Effect3D";
pub(super) const FIGURE_CLASS: &str = "com/mascotcapsule/micro3d/v3/Figure";
pub(super) const FIGURE_LAYOUT_CLASS: &str = "com/mascotcapsule/micro3d/v3/FigureLayout";
pub(super) const LIGHT_CLASS: &str = "com/mascotcapsule/micro3d/v3/Light";
pub(super) const TEXTURE_CLASS: &str = "com/mascotcapsule/micro3d/v3/Texture";
pub(super) const UTIL_3D_CLASS: &str = "com/mascotcapsule/micro3d/v3/Util3D";
pub(super) const VECTOR_3D_CLASS: &str = "com/mascotcapsule/micro3d/v3/Vector3D";
