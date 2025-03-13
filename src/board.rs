use crate::utils::substract_vec;
use itertools::Itertools;

type Position = (i32, i32);
type Table = Vec<Vec<Cell>>;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cell {
    i: usize,
    j: usize,
    pub prob_alive: f64,
}

#[derive(PartialEq, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    table: Table,
    memory: Vec<Table>,
}

impl Board {
    pub fn new(probabilities: Vec<Vec<f64>>) -> Self {
        let height = probabilities.len();
        let width = probabilities[0].len();
        let mut table = vec![];

        for i in 0..height {
            table.push(vec![]);
            for j in 0..width {
                table[i].push(Cell {
                    i,
                    j,
                    prob_alive: probabilities[i][j],
                });
            }
        }

        let memory = vec![table.clone()];

        Board {
            height,
            width,
            table,
            memory,
        }
    }

    fn get_cell(&self, (i, j): Position) -> &Cell {
        let i_usize = i.rem_euclid(self.height as i32);
        let j_usize = j.rem_euclid(self.width as i32);
        &self.table[i_usize as usize][j_usize as usize]
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
        let mut result = 0.0;
        for positive_combination in combinations {
            let negative_combination: Vec<f64> = substract_vec(neighbours, &positive_combination)
                .iter()
                .map(|x| 1.0 - x.prob_alive)
                .collect();
            result += positive_combination
                .into_iter()
                .map(|x| x.prob_alive)
                .product::<f64>()
                * negative_combination.iter().product::<f64>();
        }
        result
    }

    fn get_next_turn(&self, position: Position) -> f64 {
        let neighbours = self.get_neighbour_cells(position);

        let prob_of_2_neighbours_alive = self.get_probability_of_n_alive(&neighbours, 2);
        let prob_of_3_neighbours_alive = self.get_probability_of_n_alive(&neighbours, 3);
        let prob_of_2_or_3_neighbours_alive =
            prob_of_2_neighbours_alive + prob_of_3_neighbours_alive;

        let prob_of_alive = self.get_cell(position).prob_alive;
        let prob_of_dead = 1.0 - prob_of_alive;

        prob_of_alive * prob_of_2_or_3_neighbours_alive + prob_of_dead * prob_of_3_neighbours_alive
    }

    pub fn next(&mut self) {
        let mut table = self.table.clone();
        for (i, row) in table.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                cell.prob_alive = self.get_next_turn((i as i32, j as i32));
            }
        }
        self.table = table;
        self.memory.push(self.table.clone());
    }

    pub fn get_memory(&self) -> &Vec<Table> {
        &self.memory
    }
}
