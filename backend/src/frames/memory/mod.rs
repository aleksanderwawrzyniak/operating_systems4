mod page;
pub mod step;
pub mod utils;

use page::Page;
use step::Step;
use utils::does_contain;

use rand::prelude::*;

enum Hit {
    Hit(usize, usize),
    // hit position, to replace position
    Miss(usize, Option<usize>), //replace position, None/to replace position
}

pub struct Memory {
    pages: Vec<Option<Page>>,
    references: Vec<u64>,
    step_number: usize,
    prepage_steps: usize,
    last_used_position: usize,
    no_of_pages: usize,
    steps: Vec<Step>,
    page_hits: u64,
    page_misses: u64,
    overall_fault_percentage: f64,
    interval_start: usize,
    in_interval_misses: u64,
    pub interval_fault_percentage: f64,
}

impl Memory {
    pub fn new(no_of_pages: usize, references: Vec<u64>) -> Self {
        Memory {
            pages: {
                let mut vec = Vec::with_capacity(no_of_pages);
                for _ in 0..no_of_pages {
                    vec.push(None);
                }
                vec
            },
            references,
            step_number: 0,
            prepage_steps: 0,
            last_used_position: no_of_pages - 1,
            no_of_pages,
            steps: Vec::new(),
            page_hits: 0,
            page_misses: 0,
            overall_fault_percentage: 0.0,
            interval_start: 0,
            in_interval_misses: 0,
            interval_fault_percentage: 0.0,
        }
    }

    pub fn simulate_fifo_step(&mut self) {
        if self.step_number == self.references.len() {
            return;
        }

        let reference = self.references[self.step_number];
        self.step_number += 1;
        let pos = self.next_pos();
        let mut position: Option<usize> = None;

        if self.find_reference(reference, &mut position) {
            self.page_hits += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Hit(position.unwrap(), pos), reference);
            return;
        }

        self.page_misses += 1;
        self.in_interval_misses += 1;
        self.calculate_fault_percentages();
        self.pages[pos] = match self.pages[pos] {
            Some(_) => {
                self.push_step(Hit::Miss(pos, None), reference);
                Some(Page::new(reference))
            }
            None => {
                self.push_step(Hit::Miss(pos, None), reference);
                Some(Page::new(reference))
            }
        };

        self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        self.last_used_position = pos;
    }

    // FIFO First In First Out
    pub fn simulate_fifo_all(&mut self) {
        let end_bound = self.references.len() - self.step_number;
        for _ in 0..end_bound {
            self.simulate_fifo_step();
        }
    }

    pub fn simulate_alru_step(&mut self) {
        if self.step_number == self.references.len() {
            return;
        }

        let reference = self.references[self.step_number];
        self.step_number += 1;
        let mut position: Option<usize> = None;
        let mut pos = self.next_pos();

        if self.find_reference(reference, &mut position) {
            self.page_hits += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Hit(position.unwrap(), pos), reference);
            return;
        }

        self.page_misses += 1;
        self.in_interval_misses += 1;
        self.calculate_fault_percentages();
        match self.pages[pos] {
            Some(page) => {
                if page.is_taken() {
                    self.last_used_position = pos;
                    pos = self.next_pos();
                    self.push_step(Hit::Miss(pos, Some(self.last_used_position)), reference);
                    self.pages[pos] = Some(Page::new_taken(reference, 1_u64));
                } else {
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new_taken(reference, 1_u64));
                }
            }
            None => {
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new_taken(reference, 1_u64));
            }
        }

        self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        self.last_used_position = pos;
    }

    // ALRU
    pub fn simulate_alru_all(&mut self) {
        let end_bound = self.references.len() - self.step_number;
        for _ in 0..end_bound {
            self.simulate_alru_step();
        }
    }

    pub fn simulate_opt_step(&mut self) {
        if self.step_number == self.references.len() {
            return;
        }

        let step = self.step_number;
        let reference = self.references[step];
        self.step_number += 1;
        let mut position: Option<usize> = None;
        let mut pos = self.next_pos();

        if self.find_reference_lru(reference, &mut position) {
            self.page_hits += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Hit(position.unwrap(), pos), reference);
            return;
        }

        self.page_misses += 1;
        self.in_interval_misses += 1;
        self.calculate_fault_percentages();
        match self.pages[pos] {
            Some(_) => {
                pos = self.find_pos_with_longest_length(step, &self.references);
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new(reference));
            }
            None => {
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new(reference));
            }
        }

        self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        self.last_used_position = pos;
    }

    // OPT optimal
    pub fn simulate_opt_all(&mut self) {
        let end_bound = self.references.len() - self.step_number;
        println!("{}", end_bound);
        for _ in 0..end_bound {
            self.simulate_opt_step();
        }
    }

    pub fn simulate_rand_step(&mut self, rand: &mut ThreadRng) {
        if self.step_number == self.references.len() {
            return;
        }

        let reference = self.references[self.step_number];
        self.step_number += 1;
        let mut position: Option<usize> = None;
        let mut pos = self.next_pos();

        if self.find_reference(reference, &mut position) {
            self.page_hits += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Hit(position.unwrap(), pos), reference);
            return;
        }

        self.page_misses += 1;
        self.in_interval_misses += 1;
        self.calculate_fault_percentages();
        match self.pages[pos] {
            Some(_) => {
                pos = rand.gen_range(0, self.no_of_pages);
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new(reference));
            }
            None => {
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new(reference));
            }
        }

        self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        self.last_used_position = pos;
    }

    // RAND
    pub fn simulate_rand_all(&mut self, rand: &mut ThreadRng) {
        let end_bound = self.references.len() - self.step_number;
        for _ in 0..end_bound {
            self.simulate_rand_step(rand);
        }
    }

    pub fn simulate_lru_step(&mut self) {
        if self.step_number == self.references.len() {
            return;
        }

        let reference = self.references[self.step_number];
        self.step_number += 1;
        let mut position: Option<usize> = None;
        let mut pos = self.next_pos();

        if self.find_reference_lru(reference, &mut position) {
            self.page_hits += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Hit(position.unwrap(), pos), reference);
            return;
        }

        self.page_misses += 1;
        self.in_interval_misses += 1;
        self.calculate_fault_percentages();
        match self.pages[pos] {
            Some(page) => {
                if page.is_taken() {
                    self.last_used_position = pos;
                    pos = self.next_pos();
                    self.push_step(Hit::Miss(pos, Some(self.last_used_position)), reference);
                    self.pages[pos] =
                        Some(Page::new_taken(reference, self.no_of_pages as u64 - 1_u64));
                } else {
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] =
                        Some(Page::new_taken(reference, self.no_of_pages as u64 - 1_u64));
                }
            }
            None => {
                self.push_step(Hit::Miss(pos, None), reference);
                self.pages[pos] = Some(Page::new_taken(reference, self.no_of_pages as u64 - 1_u64));
            }
        }

        self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        self.last_used_position = pos;
    }

    // LRU Least Recently Used
    pub fn simulate_lru_all(&mut self) {
        let end_bound = self.references.len() - self.step_number;
        for _ in 0..end_bound {
            self.simulate_lru_step();
        }
    }

    pub fn find_pos_with_longest_length(&self, current_step: usize, refs: &[u64]) -> usize {
        let mut vec: Vec<u64> = Vec::new();
        for _ in 0..self.no_of_pages {
            vec.push(u64::max_value());
        }
        for (i, page) in self.pages.iter().enumerate() {
            for (step, &value) in refs[current_step..].iter().enumerate() {
                if page.unwrap().value() == value {
                    let length = step as u64;
                    if vec[i] > length {
                        vec[i] = length;
                        break;
                    }
                }
            }
        }
        let max = vec.iter().max().unwrap();
        vec.iter().enumerate().find(|n| n.1 == max).unwrap().0
    }

    fn next_pos(&mut self) -> usize {
        let pos = (self.last_used_position + 1) % self.no_of_pages;
        pos
    }

    fn calculate_fault_percentages(&mut self) {
        self.overall_fault_percentage =
            self.page_misses as f64 / (self.step_number + self.prepage_steps) as f64;
        self.interval_fault_percentage =
            self.in_interval_misses as f64 / (self.step_number - self.interval_start) as f64;
    }

    pub fn get_fault_percentage(&self) -> f64 {
        self.overall_fault_percentage
    }

    fn find_reference(&mut self, reference: u64, position: &mut Option<usize>) -> bool {
        let mut found = false;

        for (i, option) in self.pages.iter_mut().enumerate() {
            match option {
                Some(page) => {
                    if page.value() == reference {
                        page.set_taken(true);
                        page.set_ticks(1_u64);
                        found = true;
                        *position = Some(i);
                    } else {
                        if page.ticks() > 0 {
                            page.set_ticks(page.ticks() - 1_u64);
                        } else {
                            page.set_taken(false);
                        }
                    }
                }
                None => {}
            }
        }

        found
    }

    fn find_reference_lru(&mut self, reference: u64, position: &mut Option<usize>) -> bool {
        let mut found = false;

        for (i, option) in self.pages.iter_mut().enumerate() {
            // println!("{:?}", (i, &option));
            match option {
                Some(page) => {
                    if page.value() == reference {
                        page.set_taken(true);
                        page.set_ticks(self.no_of_pages as u64 - 1_u64);
                        found = true;
                        *position = Some(i);
                    } else {
                        if page.ticks() > 0 {
                            page.set_ticks(page.ticks() - 1_u64);
                        } else {
                            page.set_taken(false);
                        }
                    }
                }
                None => {}
            }
        }

        found
    }

    fn push_step(&mut self, hit: Hit, reference: u64) {
        let mut line = format!("[{}] ", reference);
        match hit {
            Hit::Hit(position, to_replace) => {
                line.push_str("#");
                for (i, option) in self.pages.iter().enumerate() {
                    if i == position && position == to_replace {
                        line.push_str(
                            format!(
                                " _|{}|_",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else if i == position {
                        line.push_str(
                            format!(
                                " _{}_",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else if i == to_replace {
                        line.push_str(
                            format!(
                                " |{}|",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else {
                        line.push_str(
                            format!(
                                " {}",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    }
                }
            }
            Hit::Miss(replaced, to_replace_option) => {
                line.push_str("$");
                for (i, option) in self.pages.iter().enumerate() {
                    if let Some(to_replace) = to_replace_option {
                        if i == to_replace {
                            line.push_str(
                                format!(
                                    " |{}|",
                                    match option {
                                        Some(page) => page.value().to_string(),
                                        None => "X".to_owned(),
                                    }
                                )
                                .as_ref(),
                            );
                            continue;
                        }
                    }
                    if i == replaced {
                        line.push_str(
                            format!(
                                " ||{}||",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else {
                        line.push_str(
                            format!(
                                " {}",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    }
                }
            }
        }

        self.steps.push(Step {
            pages: self.last_state(),
            algorithm_step: line,
            fault_rate: self.overall_fault_percentage,
        });
    }

    pub fn find_working_set(&self, max_pages: usize) -> (usize, usize, Vec<u64>) {
        let mut ws_size: usize = 0;
        let mut ws_interval_length: usize = 0;
        let mut refs: Vec<u64> = Vec::new();

        for &reference in &self.references[self.step_number..] {
            if does_contain(reference, &refs) {
                ws_interval_length += 1;
            } else if ws_size != max_pages && ws_interval_length < 2 * max_pages {
                refs.push(reference);
                ws_interval_length += 1;
                ws_size += 1;
            } else {
                break;
            }
        }

        (ws_size, ws_interval_length, refs)
    }

    pub fn set_pages(&mut self, no_pages: usize) {
        self.take_all_pages();

        self.no_of_pages = no_pages;
        for _ in 0..no_pages {
            self.pages.push(None);
        }
    }

    pub fn prepage_with(&mut self, refs: Vec<u64>) {
        for (i, &reference) in refs.iter().enumerate() {
            self.page_misses += 1;
            self.prepage_steps += 1;
            self.calculate_fault_percentages();
            self.push_step(Hit::Miss(i, None), reference);
            self.pages[i] = Some(Page::new(reference));
            self.steps[self.step_number + self.prepage_steps - 1].pages = self.last_state();
        }
    }

    pub fn last_state(&self) -> String {
        let mut line = String::new();
        for option in self.pages.iter() {
            line.push_str(
                format!(
                    "{}",
                    match option {
                        Some(page) => page.value().to_string(),
                        None => "X".to_owned(),
                    }
                )
                .as_ref(),
            );
            line.push(' ');
        }
        line.trim_end().to_owned()
    }

    /// returns the number of pages Memory currently has.
    pub fn page_number(&self) -> usize {
        self.pages.len()
    }

    /// return true if the Memory has more than one page and removes the last page.
    pub fn take_page(&mut self) -> bool {
        if self.no_of_pages > 1 {
            self.pages.pop();
            self.no_of_pages -= 1;
            return true;
        }
        false
    }

    /// gives one page to the Memory
    pub fn give_page(&mut self) {
        self.pages.push(None);
        self.last_used_position = self.no_of_pages;
        self.no_of_pages += 1;
    }

    pub fn is_done(&self) -> bool {
        self.step_number == self.references.len()
    }

    /// returns the number of pages the Memory had and clears the page Vector.
    pub fn take_all_pages(&mut self) -> usize {
        self.no_of_pages = 0;
        let n = self.pages.len();
        for _ in 0..n {
            self.pages.pop();
        }

        n
    }

    pub fn set_interval_start(&mut self) {
        self.interval_fault_percentage = 0.0;
        self.interval_start = self.step_number;
        self.in_interval_misses = 0;
    }

    pub fn hits(&self) -> u64 {
        self.page_hits
    }

    pub fn misses(&self) -> u64 {
        self.page_misses
    }

    pub fn outcome(&self) -> Vec<Step> {
        self.steps.clone()
    }
}
