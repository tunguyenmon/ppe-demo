use crate::{sot::SOT, task::Task};

#[derive(Debug, Clone)]
pub struct MSN{
    tasks: Vec<Task>,
    msn: u32,
    version: String,
}

impl MSN{
    pub fn new(msn: u32, version: &str, task_table: &Vec<Task>) -> Self{
        // Need to look for Version here then add version tasks
        //let tasks = Task::new(30.0, vec!["0".to_string(), "1".to_string(), "2".to_string()], version.to_string(), 1, 1.0, 10.0);
        let msn = Self{
            tasks: task_table.iter().filter(|task| task.get_version() == version).cloned().collect(),
            msn,
            version: String::from(version),
        };
        msn
    }

    fn add_task(&mut self, task: Task){
        self.tasks.push(task);
    }

    pub fn get_workload(&self) -> f64{
        let mut total_workload = 0.0;	
        for task in self.tasks.iter(){
            total_workload += task.get_workload();
        }
        total_workload
    }

    pub fn get_tasks(&mut self, station: &str, sot: &SOT) -> Vec<&mut Task>{
        self.tasks
            .iter_mut()
            .filter(|task|{
                task.get_stations().contains(&String::from(station)) && task.get_sot() == &sot.get_name()
            })
            .collect()
    }

    pub fn get_sot_tasks(&mut self, sot: &SOT) -> Vec<&mut Task>{
        self.tasks
            .iter_mut()
            .filter(|task|{
                task.get_sot() == &sot.get_name()
            })
            .collect()
    }
}


#[cfg(test)]
mod tests{
}