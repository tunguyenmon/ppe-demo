mod task;
mod station;
mod msn;
mod hangar;
mod util;
mod sim;
mod sot;
mod settings;

fn main() {
    print!("Please make sure, the input data of the files with the exact names\n
output.csv\nsot.csv\nstations.csv\ntactplan.csv\ntasks.csv\n
are in the folder ./data. The columns of the files must be as follows:\n
output.csv: (Generated through this program, so no need to pay attention) \nsot.csv: sot, bc, cvat, stations\nstations.csv: station\ntactplan.csv: msn, version\ntasks.csv: station, sot, version, workload\n
Hardcoded Parameters are:
TactTime: 9.333 hours (R7.5 in 2 Shift)\nInefficiency to due to BC Absence: 8.6%\n
If you get unexpected results, please make sure the data is correct\n(e.g. task data only contains data for one hangar.)\n\nSimulation Log:\n");
    let mut sim = sim::SIM::new();

    //Need to perform settings here
    sim.add_hangar("8");

    let start = std::time::SystemTime::now();
    match sim.load_data(){
        Ok(_) => println!("{:.2}s - Data Loaded.", start.elapsed().unwrap().as_secs_f32()),
        Err(_) => println!("0 - Error at loading Data."),
    }
    sim.run().expect("- Error During Simulation Run.");

    let time_since_start = start.elapsed().unwrap().as_secs_f32();
    print!("{:.2}s - Calculation completed successfully.", time_since_start);
}
