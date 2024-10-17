use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{ Layouter, SimpleFloorPlanner, Value }, // Column 제거
    plonk::{ Advice, Circuit, Column, ConstraintSystem, Error, Selector }, // Column은 plonk 모듈에서 가져옴
    poly::Rotation,
};

pub struct CalculatorCircuit<F: FieldExt> {
    pub a: Value<F>,
    pub b: Value<F>,
}

#[derive(Clone, Debug)]
pub struct AddConfig {
    /// Advice column for `input_a` and `output`.
    input_a: Column<Advice>,
    /// Advice column for `input_b`.
    input_b: Column<Advice>,
    result: Column<Advice>,
    /// Addition Selector.
    sel_add: Selector,
}

impl<F: FieldExt> Circuit<F> for CalculatorCircuit<F> {
    type FloorPlanner = SimpleFloorPlanner;
    type Config = AddConfig;

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // advice 칼럼을 생성
        let input_a = meta.advice_column();
        let input_b = meta.advice_column();
        let result = meta.advice_column();
        let sel_add = meta.selector();

        // 덧셈 연산 제약 조건 추가
        meta.create_gate("addition", |meta| {
            let a = meta.query_advice(input_a, Rotation::cur());
            let b = meta.query_advice(input_b, Rotation::cur());
            let res = meta.query_advice(result, Rotation::cur());
            let sel = meta.query_selector(sel_add); // Selector와 제약 조건 연결

            vec![sel * (a + b - res)] // Selector가 활성화된 경우에만 제약 적용
        });

        // Config 구조체 반환
        AddConfig {
            input_a,
            input_b,
            result,
            sel_add, // Selector 반환
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "addition", // 작업 이름
            |mut region| {
                config.sel_add.enable(&mut region, 0)?;
                // a 값을 region 내에 할당
                let _a = region.assign_advice(
                    || "a",
                    config.input_a,
                    0,
                    || self.a // self.a는 이미 Value 타입이므로 바로 할당
                )?;

                // b 값을 region 내에 할당
                let _b = region.assign_advice(
                    || "b",
                    config.input_b,
                    0,
                    || self.b // self.b도 마찬가지로 바로 할당
                )?;

                // result 값을 region 내에 할당 (a + b 계산)
                region.assign_advice(
                    || "result",
                    config.result,
                    0,
                    || {
                        // a와 b 값이 존재하면 덧셈을 수행
                        self.a.zip(self.b).map(|(a, b)| a + b) // 덧셈 연산
                    }
                )?;

                Ok(())
            }
        )?;

        Ok(())
    }

    fn without_witnesses(&self) -> Self {
        todo!()
    }
}
