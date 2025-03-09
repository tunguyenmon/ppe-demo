pub struct Settings{
    tacttime: f64,
    bc_inefficiency: f64,
}

impl Settings{
    pub fn new() -> Self{
        Self {
            tacttime: 9.3,
            bc_inefficiency: 0.086
        }
    }

    pub fn set_tacttime(&mut self, tacttime: f64){
        self.tacttime = tacttime;
    }
    
    pub fn get_tacttime(&self) -> f64{
        self.tacttime
    }

    pub fn set_bc_inefficiency(&mut self, bc_inefficiency: f64){
        self.bc_inefficiency = bc_inefficiency;
    }

    pub fn get_bc_inefficiency(&self) -> f64{
        self.bc_inefficiency
    }
}