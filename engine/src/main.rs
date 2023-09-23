use macros::*;

load_script!("sample.rs");

pub trait GameObject { 
    fn update(self);
    fn start(self);
}

fn main() {
    //let s = sample::Sample {};

    //s.start();
}
