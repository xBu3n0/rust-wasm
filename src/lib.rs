use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Perceptron {
    parameters: Vec<usize>,
    weight: Vec<f64>,
    bias: f64,
    data: Vec<Vec<f64>>,
    answ_index: usize,
    step_size: f64,
    accurary: f64
}

#[wasm_bindgen]
impl Perceptron {
    #[wasm_bindgen(constructor)]
    pub fn new(params: &[usize], data: &[f64], answ_index: usize, rows: usize, cols: usize) -> Perceptron {
        Perceptron {
            parameters: params.to_vec(),
            weight: vec![0.0; params.len()],
            bias: 2.0,
            answ_index: answ_index,
            data: Self::process_data(data, rows, cols),
            step_size: 0.01,
            accurary: 0.0
        }
    }
    
    pub fn set_step_size(&mut self, step_size: f64) {
        self.step_size = step_size
    }

    fn process_data(data: &[f64], rows: usize, cols: usize) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                matrix[i][j] = data[cols * i + j]
            }
        }

        matrix
    }

    pub fn train_with_verbose(&mut self, n_iter: usize) -> String {
        let mut v = Vec::new();

        'l: for _ in 0..n_iter {
            v.push(self.eval().to_string());

            if v[v.len()-1] == "1" {
                break 'l
            }
        }

        v.join(" -> ")
    }

    pub fn train_without_verbose(&mut self, n_iter: usize) {
        'l: for _ in 0..n_iter {
            if self.eval() == 1.0 {
                break 'l
            }
        }
    }

    fn eval(&mut self) -> f64 {
        let mut sum = 0.0;

        for i in 0..self.data.len() {
            if self.update(i) {
                sum += 1.0
            }
        }

        self.accurary = sum/(self.data.len() as f64);
        self.accurary
    }

    fn update(&mut self, row_index: usize) -> bool {
        let answ = Self::activate_function((0..self.parameters.len()).fold(self.bias, |sum, acc| {
            sum + self.data[row_index][acc] * self.weight[acc]
        }));

        if answ != self.data[row_index][self.answ_index] {
            for i in 0..self.weight.len() {
                self.weight[i] -= self.step_size * self.data[row_index][self.parameters[i]] * (answ - self.data[row_index][self.answ_index])
            }

            self.bias -= self.step_size * (answ - self.data[row_index][self.answ_index]);
            
            false
        } else {
            true
        }
    }

    fn activate_function(sum: f64) -> f64 {
        if sum > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn get_perceptron(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_accuracy(&self) -> f64 {
        self.accurary
    }
}