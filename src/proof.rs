use spawn_zk_snarks::{
    proof::{
        prove,
        verify,
    },
    circuit::{
        circuit_input,
        circuit_output,
    },
};

fn main() {
    let input = circuit_input();
    let proof = prove(&input);
    let output = circuit_output(&input);
    let result = verify(&proof, &output);
    println!("{}", result);
}

pub fn circuit_input() -> Vec<u64> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}