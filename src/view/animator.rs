#![allow(dead_code)]
#![allow(unused_imports)]
use crate::view::textures::Textures;
use opengl_graphics::Texture;

pub type Frame = ([f64; 2], usize);

pub trait Animator {
    fn animate(&mut self) -> Frame;
}


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PlainAnimator {
    frame_counter: usize,
    current_frame: usize,
    animate_state:bool
}

impl PlainAnimator {
    pub fn new(frame_counter: usize, current_frame: usize) -> Self {
        Self { frame_counter, current_frame, animate_state: false }
    }
}

impl Animator for PlainAnimator {
    fn animate(&mut self) -> Frame {
        if self.current_frame == self.frame_counter {

        }

        unimplemented!()
    }
}