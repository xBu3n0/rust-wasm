use wasm_bindgen::prelude::*;
use rand::{self, Rng};

// Perceptron
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
pub fn new_perceptron_from_string(params: &[usize], data_str: String, answ_index: usize) -> Perceptron {
    let params: Vec<usize> = params.iter().filter_map(|&x| {
        if x != answ_index {
            Some(x)
        } else {
            None
        }
    }).collect();
    let data: Vec<Vec<f64>> = ProcessData::evaluate_data(data_str);
    let mut rng = rand::thread_rng();

    Perceptron {
        parameters: params.to_vec(),
        weight: (0..params.len()).map(|_| rng.gen::<f64>() - 0.5).collect(),
        bias: rng.gen::<f64>() - 0.5,
        answ_index,
        data,
        step_size: 0.01,
        accurary: 0.0
    }
}

#[wasm_bindgen]
pub fn new_perceptron_from_vec(params: &[usize], data: &[f64], answ_index: usize, rows: usize, cols: usize) -> Perceptron {
    let params: Vec<usize> = params.iter().filter_map(|&x| {
        if x != answ_index {
            Some(x)
        } else {
            None
        }
    }).collect();
    let mut rng = rand::thread_rng();

    Perceptron {
        parameters: params.to_vec(),
        weight: (0..params.len()).map(|_| rng.gen::<f64>() - 0.5).collect(),
        bias: rng.gen::<f64>() - 0.5,
        answ_index,
        data: ProcessData::vec_to_matrix(data, rows, cols),
        step_size: 0.01,
        accurary: 0.0
    }
}

#[wasm_bindgen]
impl Perceptron {
    pub fn set_step_size(&mut self, step_size: f64) {
        self.step_size = step_size
    }

    pub fn train(&mut self, n_max_iter: usize, verbose: bool) -> Vec<JsValue> {
        if verbose == false {
            let mut last_accuracy = 0.0;
            let mut n = 0;

            'l: for _ in 0..n_max_iter {
                last_accuracy = self.eval();
                n += 1;
                if last_accuracy == 1.0 {
                    break 'l
                }
            }

            let info_train: String = 
                format!("O treino finalizou, a % de acerto atingida pelo programa foi de {:.2}%", last_accuracy * 100.0);

            vec![JsValue::from(info_train), JsValue::from(n)]
        } else {
            let mut v: Vec<String> = Vec::new();
            let mut n = 0;

            'l: for _ in 0..n_max_iter {
                v.push(format!("{:.2}", self.eval()));
                n += 1;

                if v[v.len()-1] == "1.00" {
                    break 'l
                }
            }
    
            vec![JsValue::from(v.join(" -> ")), JsValue::from(n)]
        }
    }

    fn eval(&mut self) -> f64 {
        let mut sum: f64 = 0.0;

        for i in 0..self.data.len() {
            if self.update(i) {
                sum += 1.0
            }
        }

        self.accurary = sum/(self.data.len() as f64);
        self.accurary
    }

    fn update(&mut self, row_index: usize) -> bool {
        let answ: f64 = Self::activate_function((0..self.parameters.len()).fold(self.bias, |sum, acc| {
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

struct ProcessData {

}

impl ProcessData {
    fn process(s: &str) -> f64 {
        match s {
            "b" => 1.0,
            "g" => 0.0,
            "F" => 1.0,
            "M" => 2.0,
            "f" => 1.0,
            "t" => 2.0,
            "?" => 0.0,
            num => num.parse().unwrap()
        }
    }
    
    fn evaluate_data(data: String) -> Vec<Vec<f64>> {
        let rows: std::str::Split<&str> = data.split("\n");
        let mut vec_data: Vec<Vec<f64>> = Vec::new();
    
        rows.into_iter().for_each(|r| {
            let values: Vec<f64> = r.split(",").map(|s| Self::process(s)).collect();
            vec_data.push(values);
        });
    
        vec_data
    }
    
    fn vec_to_matrix(data: &[f64], rows: usize, cols: usize) -> Vec<Vec<f64>> {
        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                matrix[i][j] = data[cols * i + j]
            }
        }

        matrix
    }
}