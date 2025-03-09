use crate::msn::MSN;
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub struct Station{
    name: String,
    current_msn: Option<MSN> //Station can be empty
}

impl Station{
    pub fn new(name: &str) -> Station{
        Station{
            name: String::from(name.to_ascii_uppercase()),
            current_msn: None
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    fn set_name(&mut self, name: &str){
        self.name = String::from(name.to_ascii_uppercase());
    }

    pub fn get_current_msn(&self) -> Option<&MSN>{
        //return current msn
        self.current_msn.as_ref()
    }

    pub fn get_msn_mut(&mut self) -> &mut Option<MSN>{
        &mut self.current_msn
    }

    pub fn set_current_msn(&mut self, msn: MSN){
        //set current msn
        self.current_msn = Some(msn);
    }

    pub fn tact_msn(&mut self, next_station: &mut Station){
        //Simply push MSN back one spot
        let msn = self.current_msn.take();
        //println!("Station {} is working on MSN", self.get_station_name());
        if let Some(msn) = msn{
            next_station.current_msn = Some(msn);
        }
    }

    pub fn msn_to_sink(&mut self) {
        self.current_msn = None;
    }
    
}

impl fmt::Display for Station{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tets{
    use crate::util;

    use super::*;

    #[test]
    fn tacting_works(){
        let mut this_station = Station::new("B");
        let mut previous_station = Station::new("A");

        let msn = util::get_tactplan()[0].to_owned();
        previous_station.set_current_msn(msn);

        //Trying to tact from A->B
        previous_station.tact_msn(&mut this_station);

        assert!(this_station.current_msn.is_some());
        assert!(previous_station.current_msn.is_none());
    }

    #[test]
    fn tacting_no_impact_on_workload(){
        let mut this_station = Station::new("B");
        let mut previous_station = Station::new("A");

        let msn = util::get_tactplan()[0].to_owned();
        let workload = msn.get_workload();

        previous_station.set_current_msn(msn);
        previous_station.tact_msn(&mut this_station);

        assert!(workload == this_station.get_current_msn().as_ref().unwrap().get_workload());
    }
    

}