pub mod io;
pub mod khronos;
pub mod lcdui;
pub mod location;
pub mod m2g;
pub mod m3g;
pub mod media;
pub mod midlet;
pub mod pim;
pub mod rms;
pub mod sensor;

pub use self::{
    io::*,
    lcdui::*,
    m2g::{ExternalResourceHandler, SVGAnimator, SVGEventListener, SVGImage, ScalableGraphics, ScalableImage},
    m3g::{
        AnimationController, AnimationTrack, Appearance, Background, Camera, CompositingMode, Fog, Graphics3D, Group, Image2D, IndexBuffer,
        KeyframeSequence, Light, Loader, Material, Mesh, MorphingMesh, Node, Object3D, PolygonMode, RayIntersection, SkinnedMesh, Sprite3D,
        Texture2D, Transform, Transformable, TriangleStripArray, VertexArray, VertexBuffer, World,
    },
    media::*,
    midlet::{MIDlet, MIDletStateChangeException},
    rms::*,
};

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(io::class_protos());
    factories.extend(khronos::class_protos());
    factories.extend(lcdui::class_protos());
    factories.extend(location::class_protos());
    factories.extend(m2g::class_protos());
    factories.extend(m3g::class_protos());
    factories.extend(media::class_protos());
    factories.extend(midlet::class_protos());
    factories.extend(pim::class_protos());
    factories.extend(rms::class_protos());
    factories.extend(sensor::class_protos());
    factories
}
