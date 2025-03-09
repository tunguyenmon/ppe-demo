use crate::task::Task;
#[derive(Debug)]
#[derive(Clone)]
pub struct SOT{
    name: String,
    station_assignment: Vec<String>,
    workers: u16,
    cvat: f64,
    utilization: Vec<f64>,
}

impl SOT{
    pub fn new(name: &str, station_assignment: Vec<String>, workers: u16, cvat: f64) -> SOT{
        SOT{
            station_assignment,
            workers,
            cvat,
            name : String::from(name),
            utilization: vec![]
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn get_stations(&self) -> Vec<String>{
        self.station_assignment.clone()
    }

    pub fn work(&mut self, tasklist: Vec<&mut Task>, tacttime: f64) -> f64{
        let mut available_time = tacttime;
        //Try to set priority on tasks
        for task in tasklist{
            if available_time > 0.0 {
                available_time = task.work_and_get_remaining_time(available_time, self.workers, self.cvat);
            }
            else{
                break;
            }
        }
        available_time
    }

    pub fn get_utilization(&self) -> Vec<f64>{
        self.utilization.clone()
    }

    pub fn add_utilization(&mut self, util: f64){
        self.utilization.push(util);
    }
}

#[cfg(test)]
mod tests{
}