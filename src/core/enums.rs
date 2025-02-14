use gl::types::GLuint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cap {
    Blend,
    ClipDistance0,
    ClipDistance1,
    ClipDistance2,
    ClipDistance3,
    ClipDistance4,
    ClipDistance5,
    ClipDistance6,
    ClipDistance7,
    ColorLogicOp,
    CullFace,
    DebugOutput,
    DebugOutputSynchronous,
    DepthClamp,
    DepthTest,
    Dither,
    FramebufferSrgb,
    LineSmooth,
    Multisample,
    PolygonOffsetFill,
    PolygonOffsetLine,
    PolygonOffsetPoint,
    PolygonSmooth,
    PrimitiveRestart,
    PrimitiveRestartFixedIndex,
    RasterizerDiscard,
    SampleAlphaToCoverage,
    SampleAlphaToOne,
    SampleCoverage,
    SampleShading,
    SampleMask,
    ScissorTest,
    StencilTest,
    TextureCubeMapSeamless,
    ProgramPointSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Target {
    Array,
    AtomicCounter,
    CopyRead,
    CopyWrite,
    DispatchIndirect,
    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    Query,
    ShaderStorage,
    Texture,
    TransformFeedback,
    Uniform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Usage {
    StaticDraw,
    StaticRead,
    StaticCopy,
    StreamDraw,
    StreamRead,
    StreamCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

pub struct Mask;

impl Mask {
    pub const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    pub const DEPTH_BUFFER_BIT: u32 = gl::DEPTH_BUFFER_BIT;
    pub const STENCIL_BUFFER_BIT: u32 = gl::STENCIL_BUFFER_BIT;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderType {
    Vertex,
    Fragmet,
    Geometry,
    TessControl,
    TessEvaluation,
    Compute,
}

use gl::types::{GLenum, GLfloat, GLint};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DepthMode {
    Component,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompareFunc {
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Equal,
    NotEqual,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompareMode {
    CompareRefToTexture,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MagFilter {
    Nearest,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Swizzle {
    Red,
    Green,
    Blue,
    Alpha,
    Zero,
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Wrap {
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Repeat,
    MirrorClampToEdge,
}

pub enum TexParam {
    DepthStencilMode(DepthMode),
    BaseLevel(i32),
    CompareFunc(CompareFunc),
    CompareMode(CompareMode),
    LodBias(f32),
    MinFilter(MinFilter),
    MagFilter(MagFilter),
    MinLod(f32),
    MaxLod(f32),
    MaxLevel(i32),
    SwizzleR(Swizzle),
    SwizzleG(Swizzle),
    SwizzleB(Swizzle),
    SwizzleA(Swizzle),
    SwizzleRGBA(Swizzle, Swizzle, Swizzle, Swizzle),
    WrapS(Wrap),
    WrapT(Wrap),
    WrapR(Wrap),
}

pub(super) enum TexParamPair {
    GLf(GLenum, GLfloat),
    GLi(GLenum, GLint),
    GLiv(GLenum, [GLint; 4]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TexTarget {
    Tex1D,
    Tex2D,
    Tex3D,
    Tex1DArray,
    Tex2DArray,
    TexRectangle,
    TexCubeMap,
    TexCubeMapArray,
    TexBuffer,
    Tex2DMultisample,
    Tex2DMultisampleArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinmapTarget {
    Tex1D,
    Tex2D,
    Tex3D,
    Tex1DArray,
    Tex2DArray,
    TexCubeMap,
    TexCubeMapArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageTarget {
    Texture2d,
    ProxyTexture2d,
    Texture1dArray,
    ProxyTexture1dArray,
    TextureRectangle,
    ProxyTextureRectangle,
    TextureCubeMapPositiveX,
    TextureCubeMapNegativeX,
    TextureCubeMapPositiveY,
    TextureCubeMapNegativeY,
    TextureCubeMapPositiveZ,
    TextureCubeMapNegativeZ,
    ProxyTextureCubeMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    Red,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    DepthComponent,
    DepthStencil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseFormat {
    Red,
    RG,
    RGB,
    RGBA,
    DepthComponent,
    DepthStencil,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SizedFormat {
    R8,
    R8_SNORM,
    R16,
    R16_SNORM,
    RG8,
    RG8_SNORM,
    RG16,
    RG16_SNORM,
    R3_G3_B2,
    RGB4,
    RGB5,
    RGB8,
    RGB8_SNORM,
    RGB10,
    RGB12,
    RGB16_SNORM,
    RGBA2,
    RGBA4,
    RGB5_A1,
    RGBA8,
    RGBA8_SNORM,
    RGB10_A2,
    RGB10_A2UI,
    RGBA12,
    RGBA16,
    SRGB8,
    SRGB8_ALPHA8,
    R16F,
    RG16F,
    RGB16F,
    RGBA16F,
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R11F_G11F_B10F,
    RGB9_E5,
    R8I,
    R8UI,
    R16I,
    R16UI,
    R32I,
    R32UI,
    RG8I,
    RG8UI,
    RG16I,
    RG16UI,
    RG32I,
    RG32UI,
    RGB8I,
    RGB8UI,
    RGB16I,
    RGB16UI,
    RGB32I,
    RGB32UI,
    RGBA8I,
    RGBA8UI,
    RGBA16I,
    RGBA16UI,
    RGBA32I,
    RGBA32UI,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompressedFormat {
    RED,
    RG,
    RGB,
    RGBA,
    SRGB,
    SRGB_ALPHA,
    RED_RGTC1,
    SIGNED_RED_RGTC1,
    RG_RGTC2,
    SIGNED_RG_RGTC2,
    RGBA_BPTC_UNORM,
    SRGB_ALPHA_BPTC_UNORM,
    RGB_BPTC_SIGNED_FLOAT,
    RGB_BPTC_UNSIGNED_FLOAT,
}

pub enum InternalFormat {
    Base(BaseFormat),
    Sized(SizedFormat),
    Compressed(CompressedFormat),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelDataType {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    HalfFloat,
    u8_3_3_2,
    u8_2_3_3_REV,
    u16_5_6_5,
    u16_5_6_5_REV,
    u16_4_4_4_4,
    u16_4_4_4_4_REV,
    u16_5_5_5_1,
    u16_1_5_5_5_REV,
    u32_8_8_8_8,
    u32_8_8_8_8_REV,
    u32_10_10_10_2,
    u32_2_10_10_10_REV,
}

pub type DepthFunc = CompareFunc;
pub type StencilFunc = CompareFunc;

impl TexParam {
    const fn swizzle(swizzle: Swizzle) -> i32 {
        match swizzle {
            Swizzle::Red => gl::RED as _,
            Swizzle::Green => gl::GREEN as _,
            Swizzle::Blue => gl::BLUE as _,
            Swizzle::Alpha => gl::ALPHA as _,
            Swizzle::Zero => gl::ZERO as _,
            Swizzle::One => gl::ONE as _,
        }
    }

    const fn wrap(wrap: Wrap) -> i32 {
        match wrap {
            Wrap::ClampToEdge => gl::CLAMP_TO_EDGE as _,
            Wrap::ClampToBorder => gl::CLAMP_TO_BORDER as _,
            Wrap::MirroredRepeat => gl::MIRRORED_REPEAT as _,
            Wrap::Repeat => gl::REPEAT as _,
            Wrap::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE as _,
        }
    }

    pub(super) const fn to_pair(&self) -> TexParamPair {
        match self {
            TexParam::DepthStencilMode(depth_mode) => match depth_mode {
                DepthMode::Component => {
                    TexParamPair::GLi(gl::DEPTH_STENCIL_TEXTURE_MODE, gl::DEPTH_COMPONENT as _)
                }
                DepthMode::Index => {
                    TexParamPair::GLi(gl::DEPTH_STENCIL_TEXTURE_MODE, gl::STENCIL_INDEX as _)
                }
            },
            TexParam::BaseLevel(level) => TexParamPair::GLi(gl::TEXTURE_BASE_LEVEL, *level),
            TexParam::CompareFunc(compare_func) => {
                TexParamPair::GLi(gl::TEXTURE_COMPARE_FUNC, compare_func.to_gl_func() as _)
            }
            TexParam::CompareMode(compare_mode) => match compare_mode {
                CompareMode::CompareRefToTexture => {
                    TexParamPair::GLi(gl::TEXTURE_COMPARE_MODE, gl::COMPARE_REF_TO_TEXTURE as _)
                }
                CompareMode::None => TexParamPair::GLi(gl::TEXTURE_COMPARE_MODE, gl::NONE as _),
            },
            TexParam::LodBias(value) => TexParamPair::GLf(gl::TEXTURE_LOD_BIAS, *value),
            TexParam::MinFilter(min_filter) => match min_filter {
                MinFilter::Nearest => TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST as _),
                MinFilter::Linear => TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR as _),
                MinFilter::NearestMipmapNearest => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as _)
                }
                MinFilter::LinearMipmapNearest => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as _)
                }
                MinFilter::NearestMipmapLinear => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as _)
                }
                MinFilter::LinearMipmapLinear => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as _)
                }
            },
            TexParam::MagFilter(mag_filter) => match mag_filter {
                MagFilter::Nearest => TexParamPair::GLi(gl::TEXTURE_MAG_FILTER, gl::NEAREST as _),
                MagFilter::Linear => TexParamPair::GLi(gl::TEXTURE_MAG_FILTER, gl::LINEAR as _),
            },
            TexParam::MinLod(value) => TexParamPair::GLf(gl::TEXTURE_MIN_LOD, *value),
            TexParam::MaxLod(value) => TexParamPair::GLf(gl::TEXTURE_MAX_LOD, *value),
            TexParam::MaxLevel(value) => TexParamPair::GLi(gl::TEXTURE_MAX_LEVEL, *value),
            TexParam::SwizzleR(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_R, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleG(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_G, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleB(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_B, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleA(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_A, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleRGBA(sr, sg, sb, sa) => TexParamPair::GLiv(
                gl::TEXTURE_SWIZZLE_RGBA,
                [
                    Self::swizzle(*sr),
                    Self::swizzle(*sg),
                    Self::swizzle(*sb),
                    Self::swizzle(*sa),
                ],
            ),
            TexParam::WrapS(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_S, Self::wrap(*wrap)),
            TexParam::WrapT(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_T, Self::wrap(*wrap)),
            TexParam::WrapR(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_R, Self::wrap(*wrap)),
        }
    }
}

impl TexTarget {
    pub(super) const fn to_gl_target(&self) -> u32 {
        match self {
            TexTarget::Tex1D => gl::TEXTURE_1D,
            TexTarget::Tex2D => gl::TEXTURE_2D,
            TexTarget::Tex3D => gl::TEXTURE_3D,
            TexTarget::Tex1DArray => gl::TEXTURE_1D_ARRAY,
            TexTarget::Tex2DArray => gl::TEXTURE_1D_ARRAY,
            TexTarget::TexRectangle => gl::TEXTURE_RECTANGLE,
            TexTarget::TexCubeMap => gl::TEXTURE_CUBE_MAP,
            TexTarget::TexCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TexTarget::TexBuffer => gl::TEXTURE_BUFFER,
            TexTarget::Tex2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            TexTarget::Tex2DMultisampleArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}

impl MinmapTarget {
    #[allow(dead_code)]
    pub(super) const fn to_gl_target(&self) -> u32 {
        match self {
            MinmapTarget::Tex1D => gl::TEXTURE_1D,
            MinmapTarget::Tex2D => gl::TEXTURE_2D,
            MinmapTarget::Tex3D => gl::TEXTURE_3D,
            MinmapTarget::Tex1DArray => gl::TEXTURE_1D_ARRAY,
            MinmapTarget::Tex2DArray => gl::TEXTURE_1D_ARRAY,
            MinmapTarget::TexCubeMap => gl::TEXTURE_CUBE_MAP,
            MinmapTarget::TexCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
        }
    }
}

impl Target {
    pub(super) const fn to_gl_target(self) -> GLuint {
        match self {
            Target::Array => gl::ARRAY_BUFFER,
            Target::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            Target::CopyRead => gl::COPY_READ_BUFFER,
            Target::CopyWrite => gl::COPY_WRITE_BUFFER,
            Target::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            Target::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            Target::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            Target::PixelPack => gl::PIXEL_PACK_BUFFER,
            Target::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            Target::Query => gl::QUERY_BUFFER,
            Target::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            Target::Texture => gl::TEXTURE_BUFFER,
            Target::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            Target::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

impl Usage {
    pub(super) const fn to_gl_usage(self) -> GLuint {
        match self {
            Usage::StaticDraw => gl::STATIC_DRAW,
            Usage::StaticRead => gl::STATIC_READ,
            Usage::StaticCopy => gl::STATIC_COPY,
            Usage::StreamDraw => gl::STREAM_DRAW,
            Usage::StreamRead => gl::STREAM_READ,
            Usage::StreamCopy => gl::STREAM_COPY,
            Usage::DynamicDraw => gl::DYNAMIC_DRAW,
            Usage::DynamicRead => gl::DYNAMIC_READ,
            Usage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

impl Mode {
    pub(crate) const fn to_gl_mode(self) -> u32 {
        match self {
            Mode::Points => gl::POINTS,
            Mode::LineStrip => gl::LINE_STRIP,
            Mode::LineLoop => gl::LINE_LOOP,
            Mode::Lines => gl::LINES,
            Mode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            Mode::LinesAdjacency => gl::LINES_ADJACENCY,
            Mode::TriangleStrip => gl::TRIANGLE_STRIP,
            Mode::TriangleFan => gl::TRIANGLE_FAN,
            Mode::Triangles => gl::TRIANGLES,
            Mode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            Mode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Mode::Patches => gl::PATCHES,
        }
    }
}

impl Cap {
    pub(crate) const fn to_gl_cap(self) -> u32 {
        match self {
            Cap::Blend => gl::BLEND,
            Cap::ClipDistance0 => gl::CLIP_DISTANCE0,
            Cap::ClipDistance1 => gl::CLIP_DISTANCE1,
            Cap::ClipDistance2 => gl::CLIP_DISTANCE2,
            Cap::ClipDistance3 => gl::CLIP_DISTANCE3,
            Cap::ClipDistance4 => gl::CLIP_DISTANCE4,
            Cap::ClipDistance5 => gl::CLIP_DISTANCE5,
            Cap::ClipDistance6 => gl::CLIP_DISTANCE6,
            Cap::ClipDistance7 => gl::CLIP_DISTANCE7,
            Cap::ColorLogicOp => gl::COLOR_LOGIC_OP,
            Cap::CullFace => gl::CULL_FACE,
            Cap::DebugOutput => gl::DEBUG_OUTPUT,
            Cap::DebugOutputSynchronous => gl::DEBUG_OUTPUT_SYNCHRONOUS,
            Cap::DepthClamp => gl::DEPTH_CLAMP,
            Cap::DepthTest => gl::DEPTH_TEST,
            Cap::Dither => gl::DITHER,
            Cap::FramebufferSrgb => gl::FRAMEBUFFER_SRGB,
            Cap::LineSmooth => gl::LINE_SMOOTH,
            Cap::Multisample => gl::MULTISAMPLE,
            Cap::PolygonOffsetFill => gl::POLYGON_OFFSET_FILL,
            Cap::PolygonOffsetLine => gl::POLYGON_OFFSET_LINE,
            Cap::PolygonOffsetPoint => gl::POLYGON_OFFSET_POINT,
            Cap::PolygonSmooth => gl::POLYGON_SMOOTH,
            Cap::PrimitiveRestart => gl::PRIMITIVE_RESTART,
            Cap::PrimitiveRestartFixedIndex => gl::PRIMITIVE_RESTART_FIXED_INDEX,
            Cap::RasterizerDiscard => gl::RASTERIZER_DISCARD,
            Cap::SampleAlphaToCoverage => gl::SAMPLE_ALPHA_TO_COVERAGE,
            Cap::SampleAlphaToOne => gl::SAMPLE_ALPHA_TO_ONE,
            Cap::SampleCoverage => gl::SAMPLE_COVERAGE,
            Cap::SampleShading => gl::SAMPLE_SHADING,
            Cap::SampleMask => gl::SAMPLE_MASK,
            Cap::ScissorTest => gl::SCISSOR_TEST,
            Cap::StencilTest => gl::STENCIL_TEST,
            Cap::TextureCubeMapSeamless => gl::TEXTURE_CUBE_MAP_SEAMLESS,
            Cap::ProgramPointSize => gl::PROGRAM_POINT_SIZE,
        }
    }
}

impl ShaderType {
    pub(crate) const fn to_gl_type(self) -> GLuint {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragmet => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        }
    }
}

impl ImageTarget {
    pub(super) const fn to_gl_target(self) -> GLuint {
        match self {
            ImageTarget::Texture2d => gl::TEXTURE_2D,
            ImageTarget::ProxyTexture2d => gl::PROXY_TEXTURE_2D,
            ImageTarget::Texture1dArray => gl::TEXTURE_1D_ARRAY,
            ImageTarget::ProxyTexture1dArray => gl::PROXY_TEXTURE_1D_ARRAY,
            ImageTarget::TextureRectangle => gl::TEXTURE_RECTANGLE,
            ImageTarget::ProxyTextureRectangle => gl::PROXY_TEXTURE_RECTANGLE,
            ImageTarget::TextureCubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            ImageTarget::TextureCubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            ImageTarget::TextureCubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            ImageTarget::TextureCubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            ImageTarget::TextureCubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            ImageTarget::TextureCubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            ImageTarget::ProxyTextureCubeMap => gl::PROXY_TEXTURE_CUBE_MAP,
        }
    }
}

impl BaseFormat {
    pub(super) const fn to_gl_format(self) -> GLuint {
        match self {
            BaseFormat::Red => gl::RED,
            BaseFormat::RG => gl::RG,
            BaseFormat::RGB => gl::RGB,
            BaseFormat::RGBA => gl::RGBA,
            BaseFormat::DepthComponent => gl::DEPTH_COMPONENT,
            BaseFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

impl SizedFormat {
    pub(super) const fn to_gl_format(self) -> GLuint {
        match self {
            SizedFormat::R8 => gl::R8,
            SizedFormat::R8_SNORM => gl::R8_SNORM,
            SizedFormat::R16 => gl::R16,
            SizedFormat::R16_SNORM => gl::R16_SNORM,
            SizedFormat::RG8 => gl::RG8,
            SizedFormat::RG8_SNORM => gl::RG8_SNORM,
            SizedFormat::RG16 => gl::RG16,
            SizedFormat::RG16_SNORM => gl::RG16_SNORM,
            SizedFormat::R3_G3_B2 => gl::R3_G3_B2,
            SizedFormat::RGB4 => gl::RGB4,
            SizedFormat::RGB5 => gl::RGB5,
            SizedFormat::RGB8 => gl::RGB8,
            SizedFormat::RGB8_SNORM => gl::RGB8_SNORM,
            SizedFormat::RGB10 => gl::RGB10,
            SizedFormat::RGB12 => gl::RGB12,
            SizedFormat::RGB16_SNORM => gl::RGB16_SNORM,
            SizedFormat::RGBA2 => gl::RGBA2,
            SizedFormat::RGBA4 => gl::RGBA4,
            SizedFormat::RGB5_A1 => gl::RGB5_A1,
            SizedFormat::RGBA8 => gl::RGBA8,
            SizedFormat::RGBA8_SNORM => gl::RGBA8_SNORM,
            SizedFormat::RGB10_A2 => gl::RGB10_A2,
            SizedFormat::RGB10_A2UI => gl::RGB10_A2UI,
            SizedFormat::RGBA12 => gl::RGBA12,
            SizedFormat::RGBA16 => gl::RGBA16,
            SizedFormat::SRGB8 => gl::SRGB8,
            SizedFormat::SRGB8_ALPHA8 => gl::SRGB8_ALPHA8,
            SizedFormat::R16F => gl::R16F,
            SizedFormat::RG16F => gl::RG16F,
            SizedFormat::RGB16F => gl::RGB16F,
            SizedFormat::RGBA16F => gl::RGBA16F,
            SizedFormat::R32F => gl::R32F,
            SizedFormat::RG32F => gl::RG32F,
            SizedFormat::RGB32F => gl::RGB32F,
            SizedFormat::RGBA32F => gl::RGBA32F,
            SizedFormat::R11F_G11F_B10F => gl::R11F_G11F_B10F,
            SizedFormat::RGB9_E5 => gl::RGB9_E5,
            SizedFormat::R8I => gl::R8I,
            SizedFormat::R8UI => gl::R8UI,
            SizedFormat::R16I => gl::R16I,
            SizedFormat::R16UI => gl::R16UI,
            SizedFormat::R32I => gl::R32I,
            SizedFormat::R32UI => gl::R32UI,
            SizedFormat::RG8I => gl::RG8I,
            SizedFormat::RG8UI => gl::RG8UI,
            SizedFormat::RG16I => gl::RG16I,
            SizedFormat::RG16UI => gl::RG16UI,
            SizedFormat::RG32I => gl::RG32I,
            SizedFormat::RG32UI => gl::RG32UI,
            SizedFormat::RGB8I => gl::RGB8I,
            SizedFormat::RGB8UI => gl::RGB8UI,
            SizedFormat::RGB16I => gl::RGB16I,
            SizedFormat::RGB16UI => gl::RGB16UI,
            SizedFormat::RGB32I => gl::RGB32I,
            SizedFormat::RGB32UI => gl::RGB32UI,
            SizedFormat::RGBA8I => gl::RGBA8I,
            SizedFormat::RGBA8UI => gl::RGBA8UI,
            SizedFormat::RGBA16I => gl::RGBA16I,
            SizedFormat::RGBA16UI => gl::RGBA16UI,
            SizedFormat::RGBA32I => gl::RGBA32I,
            SizedFormat::RGBA32UI => gl::RGBA32UI,
        }
    }
}

impl CompressedFormat {
    pub(super) const fn to_gl_format(self) -> GLuint {
        match self {
            CompressedFormat::RED => gl::COMPRESSED_RED,
            CompressedFormat::RG => gl::COMPRESSED_RG,
            CompressedFormat::RGB => gl::COMPRESSED_RGB,
            CompressedFormat::RGBA => gl::COMPRESSED_RGBA,
            CompressedFormat::SRGB => gl::COMPRESSED_SRGB,
            CompressedFormat::SRGB_ALPHA => gl::COMPRESSED_SRGB_ALPHA,
            CompressedFormat::RED_RGTC1 => gl::COMPRESSED_RED_RGTC1,
            CompressedFormat::SIGNED_RED_RGTC1 => gl::COMPRESSED_SIGNED_RED_RGTC1,
            CompressedFormat::RG_RGTC2 => gl::COMPRESSED_RG_RGTC2,
            CompressedFormat::SIGNED_RG_RGTC2 => gl::COMPRESSED_SIGNED_RG_RGTC2,
            CompressedFormat::RGBA_BPTC_UNORM => gl::COMPRESSED_RGBA_BPTC_UNORM,
            CompressedFormat::SRGB_ALPHA_BPTC_UNORM => gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
            CompressedFormat::RGB_BPTC_SIGNED_FLOAT => gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
            CompressedFormat::RGB_BPTC_UNSIGNED_FLOAT => gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
        }
    }
}

impl InternalFormat {
    pub(super) const fn to_gl_format(self) -> GLuint {
        match self {
            InternalFormat::Base(base_format) => base_format.to_gl_format(),
            InternalFormat::Sized(sized_format) => sized_format.to_gl_format(),
            InternalFormat::Compressed(compressed_format) => compressed_format.to_gl_format(),
        }
    }
}

impl ImageFormat {
    pub(super) const fn to_gl_format(self) -> GLuint {
        match self {
            ImageFormat::Red => gl::RED,
            ImageFormat::RG => gl::RG,
            ImageFormat::RGB => gl::RGB,
            ImageFormat::BGR => gl::BGR,
            ImageFormat::RGBA => gl::RGBA,
            ImageFormat::BGRA => gl::BGRA,
            ImageFormat::DepthComponent => gl::DEPTH_COMPONENT,
            ImageFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

impl PixelDataType {
    pub(super) const fn to_gl_type(self) -> GLuint {
        match self {
            PixelDataType::u8 => gl::UNSIGNED_BYTE,
            PixelDataType::i8 => gl::BYTE,
            PixelDataType::u16 => gl::UNSIGNED_SHORT,
            PixelDataType::i16 => gl::SHORT,
            PixelDataType::u32 => gl::UNSIGNED_INT,
            PixelDataType::i32 => gl::INT,
            PixelDataType::f32 => gl::FLOAT,
            PixelDataType::HalfFloat => gl::HALF_FLOAT,
            PixelDataType::u8_3_3_2 => gl::UNSIGNED_BYTE_3_3_2,
            PixelDataType::u8_2_3_3_REV => gl::UNSIGNED_BYTE_2_3_3_REV,
            PixelDataType::u16_5_6_5 => gl::UNSIGNED_SHORT_5_6_5,
            PixelDataType::u16_5_6_5_REV => gl::UNSIGNED_SHORT_5_6_5_REV,
            PixelDataType::u16_4_4_4_4 => gl::UNSIGNED_SHORT_4_4_4_4,
            PixelDataType::u16_4_4_4_4_REV => gl::UNSIGNED_SHORT_4_4_4_4_REV,
            PixelDataType::u16_5_5_5_1 => gl::UNSIGNED_SHORT_5_5_5_1,
            PixelDataType::u16_1_5_5_5_REV => gl::UNSIGNED_SHORT_1_5_5_5_REV,
            PixelDataType::u32_8_8_8_8 => gl::UNSIGNED_INT_8_8_8_8,
            PixelDataType::u32_8_8_8_8_REV => gl::UNSIGNED_INT_8_8_8_8_REV,
            PixelDataType::u32_10_10_10_2 => gl::UNSIGNED_INT_10_10_10_2,
            PixelDataType::u32_2_10_10_10_REV => gl::UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}

impl CompareFunc {
    pub(super) const fn to_gl_func(self) -> GLuint {
        match self {
            CompareFunc::Never => gl::NEVER,
            CompareFunc::Less => gl::LESS,
            CompareFunc::Equal => gl::EQUAL,
            CompareFunc::LessEqual => gl::LEQUAL,
            CompareFunc::Greater => gl::GREATER,
            CompareFunc::NotEqual => gl::NOTEQUAL,
            CompareFunc::GreaterEqual => gl::GEQUAL,
            CompareFunc::Always => gl::ALWAYS,
        }
    }
}
