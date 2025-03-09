use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Task{
    workload: f64,
    stations: Vec<String>,
    sot: String,
    version: String
}

impl Task{
    pub fn new(workload: f64, stations: Vec<String>, sot: String, version: String) -> Task{
        Task{
            workload,
            stations,
            sot,
            version,
        }
    }

    pub fn work_and_get_remaining_time(&mut self, available_time: f64, bluecollar: u16, cvat: f64) -> f64{
        // println!("Initial Workload: {}", self.workload);
        // println!("Blue Collars: {}", self.bluecollar);
        // println!("CVAT: {}", self.cvat);
        // println!("Available time: {}", available_time);

        // Get Available Workers with CVAT
        let available_workers = bluecollar as f64 * cvat;

        // Recalculate Task Workload by subtracting workers*time
        self.workload -= available_workers * available_time;

        //println!("Final Workload: {}", self.workload);

        //If workload goes below 0, set workload to 0 and return the
        // negative balance divided by the number of workers to get the remaining time
        if self.workload < 0.0{
            //println!("Workload complete, but available time not burned.\n");
            let remaining_time = -self.workload/available_workers;
            self.workload = 0.0;
            remaining_time
        } 
        else {
            //println!("Workload not complete, but available time burned.\n");
            0.0
        }
    }

    pub fn get_stations(&self) -> Vec<String>{
        self.stations.clone()
    }

    pub fn get_workload(&self) -> f64{
        self.workload
    }

    pub fn get_version(&self) -> &str{
        &self.version
    }

    pub fn get_sot(&self) -> &String{
        &self.sot
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_work_and_get_remaining_time(){
        let mut task = Task::new(100.0, vec!["Station1".to_string(), "Station2".to_string()], "SOT1".to_string(), "Version1".to_string());
        let mut remaining_time = task.work_and_get_remaining_time(10.0, 1, 1.);
        //Tact 9 times
        for _ in 0..9{
            remaining_time = task.work_and_get_remaining_time(10.0, 1, 1.);
        }
        assert_eq!(remaining_time, 0.0);
        assert_eq!(task.get_workload(), 0.0);
    }
}