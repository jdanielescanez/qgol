use itertools::Itertools;
use rayon::prelude::*;

type Position = (i32, i32);
type Table = Vec<Vec<Probability>>;

type Probability = f64;

#[derive(PartialEq, Debug)]
pub struct Board {
    probabilities: Table,
    survivals: Vec<usize>,
    births: Vec<usize>,
}

impl Board {
    pub fn new(probabilities: Vec<Vec<f64>>) -> Self {
        let width = probabilities[0].len();
        assert!(probabilities.iter().all(|row| row.len() == width));
        Board {
            probabilities,
            survivals: vec![2, 3],
            births: vec![3],
        }
    }

    pub fn change_rules(&mut self, survivals: Vec<usize>, births: Vec<usize>) {
        self.survivals = survivals;
        self.births = births;
    }

    pub fn height(&self) -> usize {
        self.probabilities.len()
    }

    pub fn width(&self) -> usize {
        self.probabilities.first().map(|row| row.len()).unwrap_or(0)
    }

    fn get_cell(&self, (i, j): Position) -> Probability {
        self.probabilities[i.rem_euclid(self.height() as i32) as usize]
            [j.rem_euclid(self.width() as i32) as usize]
    }

    pub fn get_state(&self) -> &Vec<Vec<Probability>> {
        &self.probabilities
    }

    fn get_neighbour_cells(&self, (i, j): Position) -> Vec<Probability> {
        let mut neighbour_positions = (i - 1..=i + 1)
            .cartesian_product(j - 1..=j + 1)
            .collect::<Vec<Position>>();
        neighbour_positions.remove(neighbour_positions.len() / 2);
        neighbour_positions
            .into_iter()
            .map(|position| self.get_cell(position))
            .collect()
    }

    fn get_probability_of_n_alive(
        &self,
        neighbours: &[Probability],
        n_alive: usize,
    ) -> Probability {
        (0..neighbours.len())
            .combinations(n_alive)
            .map(|combination| {
                neighbours
                    .iter()
                    .enumerate()
                    .map(|(index, &probability_alive)| {
                        if combination.contains(&index) {
                            probability_alive
                        } else {
                            1.0 - probability_alive
                        }
                    })
                    .product::<Probability>()
            })
            .sum()
    }

    fn get_next_turn(&self, position: Position) -> Probability {
        let neighbours = self.get_neighbour_cells(position);

        let survival_prob = self
            .survivals
            .iter()
            .map(|&survival| self.get_probability_of_n_alive(&neighbours, survival))
            .sum::<Probability>();
        let birth_prob = self
            .births
            .iter()
            .map(|&birth| self.get_probability_of_n_alive(&neighbours, birth))
            .sum::<Probability>();

        let prob_of_alive = self.get_cell(position);
        let prob_of_dead = 1.0 - prob_of_alive;

        prob_of_alive * survival_prob + prob_of_dead * birth_prob
    }

    pub fn next(&mut self) {
        let mut table = self.probabilities.clone();
        table.par_iter_mut().enumerate().for_each(|(i, row)| {
            for (j, probability_alive) in row.iter_mut().enumerate() {
                *probability_alive = self.get_next_turn((i as i32, j as i32));
            }
        });
        self.probabilities = table;
    }
}
