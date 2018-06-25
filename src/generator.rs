// TODO
// 2. 多次元対応。
// 3. PyO3 と ocl を比較。

extern crate rand;

use std;

#[derive(Debug)]
pub struct Lightning {
    pub charges: Vec<[i32; 2]>,
    pub candidate_sites: Vec<[i32; 2]>,
    potentials: Vec<f32>,
    phi: Vec<f32>,

    pub iteration: u32,
    pub eta: f32,
}

impl Lightning {
    fn add_neighbor(&mut self, position: &[i32; 2]) {
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_site = [position[0] + i, position[1] + j];

                if self.charges.contains(&new_site)
                    || self.candidate_sites.contains(&new_site)
                {
                    continue;
                };

                let potential = self.calc_new_potential(&new_site);
                self.potentials.push(potential);
                self.candidate_sites.push(new_site);
            }
        }
    }

    fn calc_new_potential(&self, site: &[i32; 2]) -> f32 {
        let mut value = 0.0;
        for charge in self.charges.iter() {
            let dx = charge[0] - site[0];
            let dy = charge[1] - site[1];
            let distance = ((dx * dx + dy * dy) as f32).sqrt();
            value += 1.0 - 0.5 / distance;
        }
        return value;
    }

    fn update_potential(&mut self, charge: &[i32; 2]) {
        for (index, site) in self.candidate_sites.iter().enumerate() {
            let dx = charge[0] - site[0];
            let dy = charge[1] - site[1];
            let distance = ((dx * dx + dy * dy) as f32).sqrt();
            self.potentials[index] += 1.0 - 0.5 / distance;
        }
    }

    pub fn grow(&mut self) -> &Lightning {
        let mut phi_min = std::f32::MAX;
        let mut phi_max = std::f32::MIN;
        let mut index = 0;
        for (i, x) in self.potentials.iter().enumerate() {
            phi_min = phi_min.min(*x);
            phi_max = phi_max.max(*x);
            if self.potentials[index] < self.potentials[i] {
                index = i;
            }
        }

        self.phi.resize(self.potentials.len(), 0.0f32);
        for (i, x) in self.potentials.iter().enumerate() {
            self.phi[i] = ((x - phi_min) / (phi_max - phi_min)).powf(self.eta);
        }

        let rnd: f32 = rand::random();
        for (i, x) in self.phi.iter().enumerate() {
            if x > &rnd && x < &self.phi[index] {
                index = i;
            }
        }

        self.potentials.remove(index);

        let charge = self.candidate_sites.remove(index);
        self.add_neighbor(&charge);
        self.update_potential(&charge);
        self.charges.push(charge);

        return self;
    }

    pub fn generate(num_iteration: u32, eta: f32) -> Lightning {
        let reserve_size = (4 * num_iteration) as usize;

        let mut lightning = Lightning {
            charges: Vec::with_capacity(num_iteration as usize),
            candidate_sites: Vec::with_capacity(reserve_size as usize),
            potentials: Vec::with_capacity(reserve_size as usize),
            phi: Vec::with_capacity(reserve_size as usize),
            iteration: num_iteration,
            eta: eta,
        };

        let pos = [0; 2];
        lightning.add_neighbor(&pos);
        lightning.charges.push(pos);

        for _ in 0..num_iteration {
            lightning.grow();
        }

        return lightning;
    }
}
