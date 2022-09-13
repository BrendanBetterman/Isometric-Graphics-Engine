use super::GameObject;

pub struct Player{
    pub game_obj: GameObject,
    pub dir: f64,
}
impl Player{
    pub fn init(&mut self){
       self.game_obj.init();

    }
    pub fn update(&mut self){
        
        if self.game_obj.transform[0] + self.dir > 200.0 || self.game_obj.transform[0] + self.dir < -200.0{
            self.dir *= -1.0;
        }
        self.game_obj.transform[0] += self.dir;
        self.game_obj.angle[1] += 0.001;
        self.game_obj.angle[2] += 0.001;
        self.game_obj.update();
    }
}