use std::fs::File;
use indexmap::IndexMap;
use csv::StringRecord;
use serde::Deserialize;
use crate::task::Task;
use crate::msn::MSN;
use crate::station::Station;
use crate::sot::SOT;


#[derive(Debug, Deserialize)]
struct TaskReadIn{
    station: String,
    sot: String,
    version: String,
    workload: f64,
}

fn read_tasks() -> Vec<Task>{
    let file_path = "data/tasks.csv";
    let file = File::open(file_path).expect("Failed to open file");

    let mut rdr = csv::Reader::from_reader(file);

    let mut task_table = vec![];
    for line in rdr.deserialize(){
        let res: TaskReadIn = line.expect("Failed to read Data");
        task_table.push(res);
    }

    translate_task(task_table)

}

fn translate_task(tasks: Vec<TaskReadIn>) -> Vec<Task>{
    let mut task_list = vec![];
    // TODO Need to add support for multiple stations in tasks
    for task in tasks{
        task_list.push(Task::new(task.workload,
            vec![task.station],
            task.sot, 
            task.version,
        ));
    }
    task_list 
}

pub fn write_to_output(osw_data: Vec<f64>){
    let mut rdr = csv::Reader::from_path("data/tactplan.csv").expect("Failed to open file");
    let mut wtr = csv::Writer::from_path("data/output.csv").expect("Failed to open file");
    
    let mut headers = rdr.headers().expect("Failed to get headers in tactplan.").clone();

    headers.push_field("osw");
    wtr.write_record(&headers).expect("Failed to write headers");


    for (i, result) in rdr.records().enumerate(){
        let mut record = result.expect("Failed to read record");

        if let Some(new_value) = osw_data.get(i){
            record.push_field(new_value.to_string().as_str());
        }
        else{
            record.push_field("");
        }

        wtr.write_record(&record).expect("Failed to write record");
    }

    wtr.flush().expect("Failed to flush writer");
}

pub fn write_sot_util(sot_util: IndexMap<String, Vec<f64>>){
    // ISSUEs: Not all SOT have the same number of tacts, need to make it only record on main tact, after all OSW has been finished.
    
    //Write a table, where each row is a tact and each column is an sot
    let mut wtr = csv::Writer::from_path("data/sot_util.csv").expect("Failed to open file");

    //Add Headers
    let mut headers: StringRecord = StringRecord::new();
    let mut sot_names: Vec<String> = vec![];
    headers.push_field("tact");
    sot_util.keys().for_each(|sot| {
        headers.push_field(sot);
        sot_names.push(sot.to_owned());
    });
    wtr.write_record(&headers).expect("Failed to write headers to sot_util.csv");

    let last_sot = sot_names.len() -1;
    let entry_length = sot_util[&sot_names[last_sot]].len();
    //println!("All Entries of last SOT: {:?}", sot_util[&sot_names[last_sot]]);
    for row in 0..entry_length{
        let mut record: StringRecord = StringRecord::new();
        record.push_field(row.to_string().as_str());
        for sot in &sot_names{
            //println!("{}", sot_util[sot][row]);
            record.push_field(sot_util[sot][row].to_string().as_str());
        }
        wtr.write_record(&record).expect("Error writing SOT to sot_util.csv");
    }

    wtr.flush().expect("Failed to flush writer for write_sot_util().");
}


#[derive(Debug, Deserialize)]
struct MSNReadIn{
    msn: u32,
    version: String,
}

pub fn get_tactplan() -> Vec<MSN>{
    let task_table = read_tasks();

    let  file_path = "data/tactplan.csv";
    let file = File::open(file_path).expect("Failed to open tactplan file");

    let mut rdr = csv::Reader::from_reader(file);

    let mut tact_plan = vec![];
    for msn in rdr.deserialize(){
        let res: MSNReadIn = msn.expect("Failed to read Data from tactplan.csv. Confirm the file has columns: msn, version\nand that all data is in the correct column.\n");
        let msn: MSN = MSN::new(res.msn, &res.version, &task_table);
        tact_plan.push(msn);
    }

    tact_plan

}

pub fn get_stations() -> Vec<Station>{
    let mut station_strings  = vec![];
    let mut stations: Vec<Station> = vec![];

    let file_path = "data/stations.csv";
    let file = File::open(file_path).expect("Failed to open file");

    let mut rdr = csv::Reader::from_reader(file);

    for station in rdr.deserialize(){
        let res: String = station.expect("Failed to read Stations");
        if !station_strings.contains(&res){
            station_strings.push(res);
            stations.push(Station::new(&station_strings[station_strings.len()-1]));
        }
    }
    stations
}

pub fn get_station_names() -> Vec<String>{
    let mut station_strings  = vec![];

    let file_path = "data/stations.csv";
    let file = File::open(file_path).expect("Failed to open file");

    let mut rdr = csv::Reader::from_reader(file);

    for station in rdr.deserialize(){
        let res: String = station.expect("Failed to read Stations");
        if !station_strings.contains(&res){
            station_strings.push(res);
        }
    }
    station_strings
}

#[derive(Debug, Deserialize)]
struct SOTReadIn{
    sot: String,
    stations: String,
    bc: u16,
    cvat: f64,
}

pub fn load_sots(sot_inefficiency: f64) -> Vec<SOT>{
    let mut sots: Vec<SOT> = vec![];
    let file_path = "data/sot.csv";
    let file = File::open(file_path).expect("Failed to open file");

    let mut rdr= csv::Reader::from_reader(file);
    for sot in rdr.deserialize(){
        let res: SOTReadIn = sot.expect("Failed to read SOT. Please check sot.csv for potential mismatches between header and data.\nAlso confirm the headers are all correctly spelled: sot, bc, cvat, stations\n");
        let stations: Vec<String> = res.stations.split(";").map(|s| s.to_string()).collect();
        for station in &stations{
            if !get_station_names().contains(station){
                panic!("Station {} not in station.csv list. Please also check sot {} in sot.csv", station, res.sot);
            }
        }
        sots.push(SOT::new(&res.sot, stations, res.bc, res.cvat-sot_inefficiency));
    }

    sots
}

/*
pub fn write_osw_to_output(osw: Vec<f64>) -> Result<(), csv::Error>{
    let mut wtr = Writer::from_path("data/output.csv")?;

    wtr.write_record(&["OSW"]).expect("Failed to write header");
    for osw in osw{
        wtr.write_record(&[osw.to_string()])?;
    }

    wtr.flush()?;

    Ok(())
}

    */

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn tasks_not_empty(){
        let tasks = read_tasks();

        assert!(tasks[0].get_stations() != [""]);
        assert!(tasks[0].get_workload() != 0.);
        //assert!(tasks[0].get_bc() == 10);
        //assert!(tasks[0].get_cvat() == 0.625);
    }

    #[test]
    fn tact_plan_not_empty(){
        let tact_plan = get_tactplan();

        assert!(tact_plan[0].get_workload() > 0.0);
    }

    #[test]
    fn stations_not_empty(){
        let stations = get_stations();

        assert!(stations[0].get_name() != "");
    }

    #[test]
    fn sot_not_empty(){
        let sots = load_sots(0.086);

        assert!(sots[0].get_name() != "");
    }
}