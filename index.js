import init, { Perceptron, new_perceptron_from_vec, new_perceptron_from_string, Wrapper, new_wrapper_from_string} from './pkg/rust_wasm.js'

await init()

const n_max_iter = 10000000
const verbose = true

// Leitura de arquivo
function readTextFile(file) {
    let rawFile = new XMLHttpRequest();
    rawFile.open("GET", file, false);

    let fileContent = "";

    rawFile.onreadystatechange = function () {
        if (rawFile.readyState === 4) {
            if (rawFile.status === 200 || rawFile.status == 0) {
                fileContent = rawFile.responseText;
            }
        }
    }

    rawFile.send(null);

    return fileContent;
}

const params = []

for (let i = 0; i < 2; ++i) {
    params.push(i)
}

const answ_index = 2
// const data = readTextFile('ionosphere.data')
const data = "2.7810836,2.550537003,0\n\
1.465489372,2.362125076,0\n\
3.396561688,4.400293529,0\n\
1.38807019,1.850220317,0\n\
3.06407232,3.005305973,0\n\
7.627531214,2.759262235,1\n\
5.332441248,2.088626775,1\n\
6.922596716,1.77106367,1\n\
8.675418651,-0.242068655,1"
// new_from_string(params: &[usize], data_str: String, answ_index: usize)
const y = new_perceptron_from_string(params, data, answ_index)
console.log(y.get_perceptron())

let init_time = Date.now()
let w = y.train(n_max_iter, verbose)
let end_time = Date.now()

console.log(w[0])
console.log('Demorou ', (end_time - init_time), 'ms para terminar a execução ', w[1], 'x, gastando ', ((end_time - init_time)) / w[1], 'ms por iteração')

// Startando o Wrapper
// new_wrapper_from_string(data_str: String, pop_len: usize, n_params: usize, answ_index: usize, n_max_iter: usize)
const wrapper = new_wrapper_from_string(data, 10, 3, 2, 10)

console.log(wrapper.get_wrapper())
const info = wrapper.train_wrapper()
console.log(info)
console.log(wrapper.get_wrapper())