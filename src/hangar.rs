use crate::{msn::MSN, sot::SOT, station::Station, task::Task};
use std::fmt;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Hangar{
    hangar: String,
    stations: Vec<Station>,
    tact_time: f64,
    sot: Vec<SOT>,
    osw: Vec<f64>,
}

impl Hangar{
    pub fn new(hangar: &str, tact_time: f64) -> Hangar{
        Hangar{
            hangar: String::from(hangar),
            stations: vec![],
            tact_time,
            sot: vec![],
            osw: vec![]
        }
    }

    #[cfg(test)]
    pub fn get_station(&self, station_name: &str) -> Option<&Station>{
        for station in self.stations.iter(){
            if station.get_name() == station_name{
                return Some(station);
            }
        }
        println!("Warning: hangar::get_station: Could not find Station");
        None
    }

    pub fn get_last_station_mut(&mut self) -> &mut Station{
        let length = self.stations.len();
        &mut self.stations[length-1]
    }

    #[cfg(test)]
    pub fn get_last_station(&self) -> &Station{
        let length = self.stations.len();
        &self.stations[length-1]
    }

    #[cfg(test)]
    pub fn get_all_stations(&self) -> &Vec<Station>{
        &self.stations
    }

    pub fn get_station_mut(&mut self, station_name: &str) -> Option<&mut Station>{
        
        for station in self.stations.iter_mut(){
            if station.get_name() == station_name{
                return Some(station);
            }
        }
        println!("Warning: hangar::get_station_mut: Could not find Station");
        None
    }

    #[cfg(test)]
    pub fn add_station(&mut self, station: Station) -> Result<(), String>{
        //Check if Station Exists
        for s in &self.stations{
            if s.get_name() == station.get_name(){
                return Err(String::from("Station already exists."));
            }
        }
        self.stations.push(station);
        Ok(())
    }

    pub fn set_stations(&mut self, stations: Vec<Station>){
        self.stations = stations;
    }

    pub fn set_sot(&mut self, sot: Vec<SOT>){
        self.sot = sot;
    }

    pub fn get_number_of_stations(&self) -> usize{
        self.stations.len()
    }

    fn calculate_osw(&mut self, len: usize){
        let osw: f64;
        if let Some(msn) = self.stations[len-1].get_current_msn().as_ref(){
            osw = msn.get_workload();
            //print!("OSW: {}", osw);
            self.osw.push(osw);
        }
    }

    fn move_all_msn(&mut self){
        // Tact all Stations after work was completed   
        let len = self.stations.len();
        self.calculate_osw(len);
        // Tact last station manually, by removing the msn (to sink)
        self.get_last_station_mut().msn_to_sink();
        // Last Station is simply overwritten
        for i in (1..len).rev(){ //Don't do first station and tact before-last station first
            let previous_station_msn = &self.stations[i-1].get_current_msn(); //Get Previous Station MSN 
            if let Some(_x) = previous_station_msn{
                //Take this station out of the array to prevent data races (Rust rules)
                let mut this_station = std::mem::replace(&mut self.stations[i], Station::new(&(i-1).to_string())); //Replace Array Entry for Station
                //print!("Tacting {}\n", this_station.get_name());
                //Tact from in Array to out of array station
                self.stations[i-1].tact_msn(&mut this_station);
                self.stations[i] = this_station; //Swap Element back
            }
        }
    }

    #[cfg(test)]
    fn get_all_sot_tasks(&mut self, sot: &SOT) -> Vec<&mut Task>{
        let mut tasklist: Vec<&mut Task> = vec![];
        for station in self.stations.iter_mut().rev(){
            let station_name = station.get_name().to_owned();
            let msn = station.get_msn_mut();
            if let Some(msn) = msn {
                tasklist.append(&mut msn.get_tasks(&station_name, sot));
            }
        }

        tasklist
    }

    fn work_on_osw(&mut self, sot: &mut SOT, initial_remaining_time: f64) -> f64{
        let mut remaining_time = initial_remaining_time;
        let station_names = sot.get_stations();
        for station_name in station_names{
            if remaining_time > 0.0{
                let station = self.get_station_mut(&station_name);
                if let Some(station) = station{
                    let msn = station.get_msn_mut();
                    if let Some(msn) = msn{
                        let tasks = msn.get_sot_tasks(sot);
                        remaining_time = sot.work(tasks, remaining_time);
                    }
                }
                else{
                    panic!("Station {} not found in hangar in hangar.tact().\nMake sure all stations required in sot.csv are also listed in stations.csv.", station_name);
                }
            }
            else{
                break;
            }
        }
        remaining_time
    }

    pub fn tact(& mut self){
        // Let All SOTs work on the stations in the hangar
        // Take SOTs out of Hangar to prevent data races (Rust rules)
        let mut sot = std::mem::replace(&mut self.sot, vec![]);
        //println!("{}", sot.len());
        for sot in &mut sot{
            //Get All tasks for that SOT in the whole hangar
            let mut tasklist: Vec<&mut Task> = vec![];
            for station in self.stations.iter_mut().rev(){
                let station_name = station.get_name().to_owned();
                let msn = station.get_msn_mut();
                if let Some(msn) = msn {
                    tasklist.append(&mut msn.get_tasks(&station_name, sot));
                }
            }
            //Work on Main Station
            let mut remaining_time = sot.work(tasklist, self.tact_time);

            //Burn OSW from previous Stations with remaining time
            if remaining_time > 0.0{
                remaining_time = self.work_on_osw(sot, remaining_time);
                
            }
            sot.add_utilization((self.tact_time - remaining_time)/self.tact_time);
        }
        self.sot = sot; //Return SOT after manipulation

        //Removes the last MSN (to sink) and moves all MSN one station further.
        self.move_all_msn();
    }

    pub fn insert_msn(&mut self, msn: MSN){
        let first_station = &mut self.stations[0];
        first_station.set_current_msn(msn);
    }

    pub fn get_osw(&self) -> &Vec<f64>{
        &self.osw
    }

    pub fn get_sot_utilization(&self) -> IndexMap<String, Vec<f64>>{
        //Get SOT Names
        let mut sot_util_map: IndexMap<String, Vec<f64>> = IndexMap::new();
        self.sot.iter().for_each(
            |sot| {sot_util_map.insert(sot.get_name(),sot.get_utilization());});

        sot_util_map
    }
}

impl fmt::Display for Hangar{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hangar: {}\n", self.hangar)?;
        for station in &self.stations{
            write!(f, "{}", station)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use crate::util;

    use super::*;

    #[test]
    fn msn_insertion_works(){
        let mut hangar = Hangar::new("1", 1.0);
        let station = Station::new("1");
        hangar.add_station(station);
        let msn = util::get_tactplan()[0].to_owned();
        hangar.insert_msn(msn);
        assert!(hangar.stations[0].get_current_msn().is_some());
    }

    #[test]
    fn tacting_works(){
        let mut hangar = Hangar::new("1", 1.0);
        hangar.set_stations(util::get_stations()[0..3].to_vec());
        let msn = util::get_tactplan()[0].to_owned();
        hangar.insert_msn(msn);
        hangar.tact();
        assert!(hangar.stations[0].get_current_msn().is_none());
        assert!(hangar.stations[1].get_current_msn().is_some());
        assert!(hangar.stations[2].get_current_msn().is_none());
    }

    #[test]
    fn osw_calculation_works(){
        let mut hangar = Hangar::new("1", 1.0);
        hangar.set_stations(util::get_stations()[0..3].to_vec());
        let msn = util::get_tactplan()[0].to_owned();
        hangar.insert_msn(msn);
        for _ in 0..3{
            hangar.tact();
        }
        println!("OSW: {:#?}", hangar.get_osw());
    }
}