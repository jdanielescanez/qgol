use crate::utils::substract_vec;
use itertools::Itertools;

type Position = (usize, usize);
type Table = Vec<Vec<Cell>>;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cell {
    pub prob_alive: f64,
}

#[derive(PartialEq, Debug)]
pub struct Board {
    table: Table,
    memory: Vec<Table>,
    survivals: Vec<usize>,
    revivals: Vec<usize>,
}

impl Board {
    pub fn new(probabilities: Vec<Vec<f64>>) -> Self {
        let width = probabilities[0].len();
        assert!(probabilities.iter().all(|row| row.len() == width));
        let table = probabilities
            .into_iter()
            .map(|row| row.into_iter().map(|f| Cell { prob_alive: f }).collect())
            .collect::<Vec<_>>();

        let memory = vec![table.clone()];

        Board {
            table,
            memory,
            survivals: vec![2, 3],
            revivals: vec![3],
        }
    }

    pub fn change_rules(&mut self, (survivals, revivals): (Vec<usize>, Vec<usize>)) {
        self.survivals = survivals;
        self.revivals = revivals;
    }

    pub fn height(&self) -> usize {
        self.table.len()
    }

    pub fn width(&self) -> usize {
        self.table.first().map(|row| row.len()).unwrap_or(0)
    }

    fn get_cell(&self, (i, j): Position) -> &Cell {
        &self.table[i.rem_euclid(self.height())][j.rem_euclid(self.width())]
    }

    fn get_neighbour_cells(&self, (i, j): Position) -> Vec<&Cell> {
        let mut neighbour_positions = (i - 1..=i + 1)
            .cartesian_product(j - 1..=j + 1)
            .collect::<Vec<Position>>();
        neighbour_positions.remove(neighbour_positions.len() / 2);
        neighbour_positions
            .into_iter()
            .map(|position| self.get_cell(position))
            .collect()
    }

    fn get_probability_of_n_alive(&self, neighbours: &[&Cell], n_alive: usize) -> f64 {
        let combinations: Vec<Vec<&Cell>> = neighbours
            .iter()
            .combinations(n_alive)
            .map(|x| x.into_iter().copied().collect())
            .collect();

        combinations
            .into_iter()
            .map(|positive_combination| {
                let negative_combination_prob: f64 =
                    substract_vec(neighbours, &positive_combination)
                        .iter()
                        .map(|x| 1.0 - x.prob_alive)
                        .product();

                let positive_combination_prob = positive_combination
                    .into_iter()
                    .map(|x| x.prob_alive)
                    .product::<f64>();
                positive_combination_prob * negative_combination_prob
            })
            .sum()
    }

    fn get_next_turn(&self, position: Position) -> f64 {
        let neighbours = self.get_neighbour_cells(position);

        let survival_prob: f64 = self
            .survivals
            .iter()
            .map(|&survival| self.get_probability_of_n_alive(&neighbours, survival))
            .sum();
        let revival_prob: f64 = self
            .revivals
            .iter()
            .map(|&revival| self.get_probability_of_n_alive(&neighbours, revival))
            .sum();

        let prob_of_alive = self.get_cell(position).prob_alive;
        let prob_of_dead = 1.0 - prob_of_alive;

        prob_of_alive * survival_prob + prob_of_dead * revival_prob
    }

    pub fn next(&mut self) {
        let mut table = self.table.clone();
        for (i, row) in table.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                cell.prob_alive = self.get_next_turn((i, j));
            }
        }
        self.table = table;
        self.memory.push(self.table.clone());
    }

    pub fn get_memory(&self) -> &Vec<Table> {
        &self.memory
    }
}
