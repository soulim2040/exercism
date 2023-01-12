#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

struct Stack {
    dt : Vec<CalculatorInput>,
}

impl Stack {
    fn new() -> Self {
        Self {
            dt : Vec::new(),
        } 
    }

    // fn from_arr(inputs: &[CalculatorInput]) -> Self {
    //     Self {
    //         dt : inputs.to_vec(),
    //     }
    // }

    fn push(&mut self, input: CalculatorInput) {
        self.dt.push(input);
    }

    fn pop(&mut self) -> CalculatorInput {
        self.dt.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.dt.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, index: usize) -> Option<&CalculatorInput>{
        self.dt.get(index)
    }

    fn try_to_cal(&mut self){
        if self.len() < 3 {
            return;
        }

        for i in 2.. self.len() {
            let left = self.get(i-2).unwrap();
            let right = self.get(i-1).unwrap();
            let op = self.get(i).unwrap();

            if let CalculatorInput::Value(left_v) = left {
                if let CalculatorInput::Value(right_v) = right {
                    let result = match op {
                        CalculatorInput::Add => Some(left_v + right_v),
                        CalculatorInput::Subtract => Some(left_v - right_v),
                        CalculatorInput::Multiply => Some(left_v * right_v),
                        CalculatorInput::Divide => {
                            if *right_v == 0 {
                                None
                            } else {
                                Some(left_v / right_v)
                            }
                        }
                        _ => None,
                    };

                    //pop three and push result
                    if let Some(vv) = result {
                        self.pop();
                        self.pop();
                        self.pop();
                        self.push(CalculatorInput::Value(vv));
                    }
                }
            }
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut st = Stack::new();

    for ch in inputs {
        st.push(*ch);
        if st.len() >= 3 {
            st.try_to_cal();
        }
    }

    if st.len() == 1 {
        let val = st.get(0).unwrap();
        if let CalculatorInput::Value(v) = *val {
            return Some(v);
        }
    }
    return None;
}
