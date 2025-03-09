use crate::hangar::Hangar;
use crate::msn::MSN;
use crate::util;
use crate::settings::Settings;

pub struct SIM{
    hangar: Vec<Hangar>,
    msns: Vec<MSN>,
    settings: Settings
}

impl SIM{
    pub fn new() -> SIM{
        SIM{
            hangar: vec![],
            msns: vec![],
            settings: Settings::new()
        }
    }

    pub fn set_tacttime(&mut self, tact_time: f64){
        self.settings.set_tacttime(tact_time);
    }

    pub fn set_bc_inefficiency(&mut self, inefficiency: f64){
        self.settings.set_bc_inefficiency(inefficiency);
    }

    pub fn add_hangar(&mut self, hangar_name: &str) -> &Hangar{
        let hangar: Hangar = Hangar::new(hangar_name, self.settings.get_tacttime());
        self.hangar.push(hangar);
        self.hangar.last().unwrap()
    }

    #[cfg(test)]
    pub fn get_hangar(&self, index: usize) -> &Hangar{
        &self.hangar[index]
    }

    pub fn get_hangar_mut(&mut self, index: usize) -> &mut Hangar{
        &mut self.hangar[index]
    }

    pub fn load_data(&mut self) -> Result<(), String>{  
        // Add Stations from CSV to Hangar
        let inefficiency = self.settings.get_bc_inefficiency();
        let hangar = self.get_hangar_mut(0);
        hangar.set_stations(util::get_stations());
        hangar.set_sot(util::load_sots(inefficiency));
        
        // Set MSNs to TactPlan
        self.msns = util::get_tactplan();
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String>{
        // Run Simulation
        let mut msns = std::mem::take(&mut self.msns);
        let hangar = self.get_hangar_mut(0);
        
        //Tact until all msn are in
        for msn in msns.drain(..){
           hangar.insert_msn(msn);
           hangar.tact();
        }

        //Tact until the end, last MSN exits hangar
        for _i in 0..hangar.get_number_of_stations(){
            hangar.tact();
        }

        let osw = hangar.get_osw().to_owned();
        util::write_to_output(osw);
        let sot_util = hangar.get_sot_utilization();
        util::write_sot_util(sot_util);

        Ok(())
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn hangar_tacts_until_end(){
        let mut sim = SIM::new();
        sim.add_hangar("1");
        sim.load_data().unwrap();
        sim.run().unwrap();
        
        //Last Station has no MSN;
        //println!("{:?}", sim.get_hangar(0).get_station("S10").unwrap().get_current_msn().unwrap());
        //println!("{:?}", sim.get_hangar(0).get_last_station().unwrap().get_current_msn().unwrap());
        println!("{:#?}", sim.get_hangar(0).get_all_stations());
        assert!(sim.get_hangar(0).get_last_station().get_current_msn().is_none())
    }
}