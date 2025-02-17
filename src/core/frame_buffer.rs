use gl::types::GLuint;

use super::{Attachmect, Filter, FrameBufferTarget, RenderBuffer};

pub struct FrameBuffer {
    fbo: GLuint,
}

impl FrameBuffer {
    #[inline]
    pub(super) fn new() -> Self {
        let mut fbo = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
        }
        Self { fbo }
    }
}

impl Drop for FrameBuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
        }
    }
}

pub struct FrameBuffers {
    fbos: Vec<FrameBuffer>,
}

impl FrameBuffers {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let mut fbos = Vec::with_capacity(count);
        unsafe {
            gl::GenFramebuffers(count as i32, fbos.as_mut_ptr());
        }
        Self {
            fbos: fbos.into_iter().map(|fbo| FrameBuffer { fbo }).collect(),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &FrameBuffer> {
        self.fbos.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FrameBuffer> {
        self.fbos.iter_mut()
    }
}

pub struct IntoIter {
    fbos: Vec<Option<FrameBuffer>>,
    index: usize,
}

impl IntoIter {
    #[inline]
    pub fn new(fbos: Vec<FrameBuffer>) -> Self {
        Self {
            fbos: fbos.into_iter().map(|fbo| Some(fbo)).collect(),
            index: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = FrameBuffer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.fbos.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for FrameBuffers {
    type Item = FrameBuffer;

    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.fbos)
    }
}

impl FrameBuffer {
    /// Wrapper of `glBindFramebuffer(...)`
    #[inline]
    pub fn bind(&self, target: FrameBufferTarget) {
        unsafe {
            gl::BindFramebuffer(target.to_gl_target(), self.fbo);
        }
    }

    /// Wrapper of `glBindFramebuffer(...)`
    #[inline]
    pub fn unbind(target: FrameBufferTarget) {
        unsafe {
            gl::BindFramebuffer(target.to_gl_target(), 0);
        }
    }

    /// Wrapper of `glFramebufferRenderbuffer(...)`
    #[inline]
    pub fn attach_render_buffer(
        target: FrameBufferTarget,
        attachment: Attachmect,
        render_buffer: &RenderBuffer,
    ) {
        unsafe {
            gl::FramebufferRenderbuffer(
                target.to_gl_target(),
                attachment.to_gl_attachment(),
                gl::RENDERBUFFER,
                render_buffer.rbo,
            );
        }
    }

    /// Wrapper of `glCheckFramebufferStatus(...)`
    #[inline]
    pub fn check_status(target: FrameBufferTarget) -> Result<(), String> {
        let err = unsafe { gl::CheckFramebufferStatus(target.to_gl_target()) };
        if err == gl::FRAMEBUFFER_COMPLETE {
            Ok(())
        } else {
            Err(format!("Not complete, error code: {}", err))
        }
    }

    /// Wrapper of `glBlitNamedFramebuffer(...)`
    pub fn blit_from(
        &self,
        frame_buffer: &FrameBuffer,
        ((src_x0, src_y0), (src_x1, src_y1)): ((u32, u32), (u32, u32)),
        ((dst_x0, dst_y0), (dst_x1, dst_y1)): ((u32, u32), (u32, u32)),
        mask: u32,
        filter: Filter,
    ) {
        unsafe {
            gl::BlitNamedFramebuffer(
                frame_buffer.fbo,
                self.fbo,
                src_x0 as _,
                src_y0 as _,
                src_x1 as _,
                src_y1 as _,
                dst_x0 as _,
                dst_y0 as _,
                dst_x1 as _,
                dst_y1 as _,
                mask,
                filter.to_gl_filter(),
            );
        }
    }

    /// Wrapper of `glBlitFramebuffer(...)`
    pub fn blit(
        ((src_x0, src_y0), (src_x1, src_y1)): ((u32, u32), (u32, u32)),
        ((dst_x0, dst_y0), (dst_x1, dst_y1)): ((u32, u32), (u32, u32)),
        mask: u32,
        filter: Filter,
    ) {
        unsafe {
            gl::BlitFramebuffer(
                src_x0 as _,
                src_y0 as _,
                src_x1 as _,
                src_y1 as _,
                dst_x0 as _,
                dst_y0 as _,
                dst_x1 as _,
                dst_y1 as _,
                mask,
                filter.to_gl_filter(),
            );
        }
    }
}
