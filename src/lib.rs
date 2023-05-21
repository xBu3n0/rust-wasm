mod perceptron;

// use perceptron::perceptron::*;
// use wasm_bindgen::prelude::*;


// #[wasm_bindgen]
// pub struct Wrapper {
//     population: Vec<Perceptron>,
//     data: Vec<Vec<f64>>,
//     n_max_iter: usize
// }

// #[wasm_bindgen]
// impl Wrapper {
//     pub fn train_wrapper(&mut self) -> f64 {
//         (0..self.population.len()).fold(0.0, |best_acc, acc| {
//             best_acc.max(self.population[acc].train(&self.data, self.n_max_iter, false)[1].as_f64().unwrap())
//         })
//     }
// }

// #[wasm_bindgen]
// pub fn new_wrapper_from_string(data_str: String, pop_len: usize, n_max_iter: usize) -> Wrapper {
//     let data: Vec<Vec<f64>> = evaluate_data(data_str);
//     let mut rng = rand::thread_rng();

//     Wrapper {
//         population: (0..pop_len).map(|_| new_perceptron_from_string(&[], 0)).collect(),
//         data: data,
//         n_max_iter: n_max_iter
//     }
// }

// #[wasm_bindgen]
// pub fn new_wrapper_from_vec(data: &[f64], pop_len: usize, n_max_iter: usize, rows: usize, cols: usize) -> Wrapper {
//     // let params: Vec<usize> = params.iter().filter_map(|&x| {
//     //     if x != answ_index {
//     //         Some(x)
//     //     } else {
//     //         None
//     //     }
//     // }).collect();
//     let mut rng = rand::thread_rng();

//     Wrapper {
//         population: (0..pop_len).map(|_| new_perceptron_to_wrapper(&[], 0)).collect(),
//         data: data,
//         n_max_iter: n_max_iter
//     }
// }

// fn evaluate_data(data: String) -> Vec<Vec<f64>> {
//     let rows: std::str::Split<&str> = data.split("\n");
    
//     rows.into_iter().fold(&mut Vec::new(), |v, r| {
//         let values: Vec<f64> = r.split(",").map(|s| process(s)).collect();
//         v.push(values);
//         v
//     }).to_owned()
// }

// fn process(s: &str) -> f64 {
//     match s {
//         "b" => 1.0,
//         "g" => 0.0,
//         "F" => 1.0,
//         "M" => 2.0,
//         "f" => 1.0,
//         "t" => 2.0,
//         "?" => 0.0,
//         num => num.parse().unwrap()
//     }
// }