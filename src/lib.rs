mod perceptron;

// use std::{rc::Rc, borrow::Borrow};
use rand::Rng;
use serde_wasm_bindgen::to_value;
use std::rc::Rc;
use wasm_bindgen::JsValue;

use perceptron::perceptron::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Wrapper {
    population: Vec<Perceptron>,
    data: Rc<Vec<Vec<f64>>>,
    n_max_iter: usize,
}

#[wasm_bindgen]
impl Wrapper {
    // Vai executar o treinando, retornando o melhor filho daqla geração
    // [[parametros do filho de melhor acuraria, acuracia_do_melhor_filho], ...]
    #[wasm_bindgen]
    pub fn train_wrapper(&mut self) -> Vec<JsValue> {
        let mut result = Vec::new();

        (0..self.population.len()).for_each(|i| {
            result.extend(self.population[i].train(self.n_max_iter, false));
        });

        result
    }

    pub fn get_wrapper(&mut self) -> String {
        self.population
            .iter()
            .map(|p| format!("{:?}", p))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

#[wasm_bindgen]
pub fn new_wrapper_from_string(
    data_str: String,
    pop_len: usize,
    n_params: usize,
    answ_index: usize,
    n_max_iter: usize,
) -> Wrapper {
    let data: Rc<Vec<Vec<f64>>> = ProcessData::evaluate_data(data_str);
    let mut rng = rand::thread_rng();

    Wrapper {
        population: (0..pop_len)
            .map(|i| {
                new_perceptron_to_wrapper(
                    Rc::clone(&data),
                    {
                        let mut params: Vec<usize> = Vec::new();
                        (0..n_params)
                            .fold(&mut params, |p, acc| {
                                if rng.gen::<bool>() {
                                    p.push(acc)
                                }
                                p
                            })
                            .to_owned()
                    },
                    answ_index,
                    0.01,
                )
            })
            .collect(),
        data: Rc::clone(&data),
        n_max_iter: n_max_iter,
    }
}

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

// // fn evaluate_data(data: String) -> Vec<Vec<f64>> {
// //     let rows: std::str::Split<&str> = data.split("\n");

// //     rows.into_iter().fold(&mut Vec::new(), |v, r| {
// //         let values: Vec<f64> = r.split(",").map(|s| process(s)).collect();
// //         v.push(values);
// //         v
// //     }).to_owned()
// // }

// // fn process(s: &str) -> f64 {
// //     match s {
// //         "b" => 1.0,
// //         "g" => 0.0,
// //         "F" => 1.0,
// //         "M" => 2.0,
// //         "f" => 1.0,
// //         "t" => 2.0,
// //         "?" => 0.0,
// //         num => num.parse().unwrap()
// //     }
// // }
