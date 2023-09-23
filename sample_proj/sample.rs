use macros::*;

pub struct Sample { 
    pub speed: f32;
}

#[game_object]
impl GameObject for Sample {
    fn start(self) {
        println!("whuh");
    }
    
    fn update(self) {
        println!("update");
    }
}

