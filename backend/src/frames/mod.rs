pub mod memory;
pub mod process;

use memory::utils::split_references;
use memory::Memory;
use process::{InputProcess, OutputProcess};

use crate::connections::Request;
use rand::prelude::*;
use rand::{thread_rng, Rng};

pub enum Algorithm {
    Fifo,
    Lru,
    Alru,
    Opt,
    Rand,
}

pub struct FrameSimulator {
    no_frames: usize,
    no_processes: usize,
    processes: Vec<InputProcess>,
    simulations: Vec<Memory>,
    interval: usize,
}

impl FrameSimulator {
    pub fn new(
        no_frames: usize,
        no_processes: usize,
        processes: Vec<InputProcess>,
        interval: usize,
    ) -> Self {
        FrameSimulator {
            no_frames,
            no_processes,
            processes,
            simulations: Vec::new(),
            interval,
        }
    }

    pub fn from_request(r: &Request) -> Self {
        FrameSimulator {
            no_frames: r.no_frames,
            no_processes: r.no_processes,
            processes: r.processes.clone(),
            simulations: Vec::new(),
            interval: r.interval,
        }
    }

    pub fn simulate_equal(&mut self, algorithm: Algorithm) {
        let frames_for_single_process: usize = self.no_frames / self.processes.len();
        for process in self.processes.iter() {
            let requests = split_references(&process.requests);
            self.simulations
                .push(Memory::new(frames_for_single_process, requests));
        }

        let mut rand = rand::thread_rng();

        for simulator in self.simulations.iter_mut() {
            simulate_all(simulator, &algorithm, &mut rand);
        }
    }

    pub fn simulate_proportional(&mut self, algorithm: Algorithm) {
        let size_sum = self.size_sum();

        for process in self.processes.iter() {
            let requests = split_references(&process.requests);
            self.simulations.push(Memory::new(
                proportional_pages(size_sum, process.size, self.no_frames),
                requests,
            ));
        }

        let mut rand = rand::thread_rng();

        for simulator in self.simulations.iter_mut() {
            simulate_all(simulator, &algorithm, &mut rand);
        }
    }

    pub fn simulate_rand(&mut self, algorithm: Algorithm) {
        let mut frames = self.no_frames;
        let mut rand = rand::thread_rng();

        for (i, process) in self.processes.iter().enumerate() {
            let requests = split_references(&process.requests);
            let upper = frames as u64 - (self.processes.len() - i - 1) as u64;
            print!("{:?} ", (1_u64, upper));
            let page_number = rand.gen_range(1_u64, upper) as usize;
            println!("{}", page_number);
            frames -= page_number as usize;
            self.simulations.push(Memory::new(page_number, requests));
        }

        for simulator in self.simulations.iter_mut() {
            simulate_all(simulator, &algorithm, &mut rand);
        }
    }

    pub fn simulate_pff(&mut self, algorithm: Algorithm) {
        let mut frames = self.no_frames;
        let mut rand = thread_rng();
        let frames_for_single_process: usize = frames / self.processes.len();
        let mut max_steps = 0_usize;

        for process in &self.processes {
            let requests = split_references(&process.requests);
            if requests.len() > max_steps {
                max_steps = requests.len();
            }
            self.simulations
                .push(Memory::new(frames_for_single_process, requests));
            frames -= frames_for_single_process;
        }

        for _ in 0..frames_for_single_process {
            for simulation in self.simulations.iter_mut() {
                simulate_step(simulation, &algorithm, &mut rand);
            }
        }

        let mut counter = 0_usize;
        let upper_bound = max_steps - frames_for_single_process;

        while counter < upper_bound {
            for _ in 0..self.interval {
                for simulator in self.simulations.iter_mut() {
                    if !simulator.is_done() {
                        simulate_step(simulator, &algorithm, &mut rand);
                    }
                }
                counter += 1;
            }

            if counter < upper_bound {
                for simulator in self.simulations.iter_mut() {
                    let percentage = simulator.interval_fault_percentage;
                    println!("{}", percentage);
                    if simulator.is_done() {
                        frames += simulator.take_all_pages();
                    }
                    if percentage <= 0.3_f64 {
                        simulator.take_page();
                        frames += 1;
                    } else if percentage >= 0.8_f64 {
                        if frames > 0 {
                            frames -= 1;
                            simulator.give_page();
                        }
                    }

                    simulator.set_interval_start();
                }
            }
        }
    }

    pub fn simulate_wsa(&mut self, algorithm: Algorithm) {
        let mut frames = self.no_frames;
        let mut rand = thread_rng();
        let max_frames = 5_usize;

        for process in &self.processes {
            let requests = split_references(&process.requests);
            self.simulations.push(Memory::new(1_usize, requests));
        }

        loop {
            for simulator in self.simulations.iter_mut() {
                let (required_frames, ws_interval_length, refs) = if frames > max_frames {
                    simulator.find_working_set(max_frames)
                } else {
                    simulator.find_working_set(frames)
                };

                simulator.set_pages(required_frames);
                frames -= required_frames;
                simulator.prepage_with(refs);

                for _ in 0..ws_interval_length {
                    simulate_step(simulator, &algorithm, &mut rand);
                }

                frames += simulator.take_all_pages();
            }

            let mut end_simulation = true;
            for simulator in self.simulations.iter() {
                if !simulator.is_done() {
                    end_simulation = false;
                    break;
                }
            }

            if end_simulation {
                break;
            }
        }
    }

    pub fn get_results(&self) -> Vec<OutputProcess> {
        let mut out: Vec<OutputProcess> = Vec::new();

        for (i, simulation) in self.simulations.iter().enumerate() {
            out.push(OutputProcess {
                id: self.processes[i].id,
                size: self.processes[i].size,
                faults: simulation.misses(),
                fault_percentage: simulation.get_fault_percentage(),
                no_frames: simulation.page_number(),
                steps: simulation.outcome(),
            });
        }

        out
    }

    fn size_sum(&self) -> usize {
        let mut sum = 0_usize;

        for process in self.processes.iter() {
            sum += process.size;
        }

        sum
    }

    pub fn get_avg_page_miss(&self) -> f64 {
        let mut sum = 0.0_f64;

        for simulator in self.simulations.iter() {
            sum += simulator.get_fault_percentage();
        }

        sum / self.no_processes as f64
    }
}

fn simulate_all(simulation: &mut Memory, algorithm: &Algorithm, rand: &mut ThreadRng) {
    use Algorithm::*;

    match algorithm {
        Fifo => simulation.simulate_fifo_all(),
        Lru => simulation.simulate_lru_all(),
        Alru => simulation.simulate_alru_all(),
        Opt => simulation.simulate_opt_all(),
        Rand => simulation.simulate_rand_all(rand),
    }
}

fn simulate_step(simulation: &mut Memory, algorithm: &Algorithm, rand: &mut ThreadRng) {
    use Algorithm::*;

    match algorithm {
        Fifo => simulation.simulate_fifo_step(),
        Lru => simulation.simulate_lru_step(),
        Alru => simulation.simulate_alru_step(),
        Opt => simulation.simulate_opt_step(),
        Rand => simulation.simulate_rand_step(rand),
    }
}

fn proportional_pages(sum: usize, size: usize, frames: usize) -> usize {
    let proportion = ((size as f64 / sum as f64) * frames as f64);
    if proportion < 1.0_f64 {
        1_usize
    } else {
        proportion.floor() as usize
    }
}
