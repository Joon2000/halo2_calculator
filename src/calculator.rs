pub mod add;

use add::CalculatorCircuit;
use halo2_proofs::{ pasta::Fp, dev::MockProver, circuit::Value };

pub fn run() -> bool {
    let circuit = CalculatorCircuit {
        a: Value::known(Fp::from(3)), // Value::known으로 값 할당
        b: Value::known(Fp::from(5)), // Value::known으로 값 할당
    };

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
    true
}
