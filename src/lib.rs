use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StackMachine {
    stack: Vec<i32>,
}

#[wasm_bindgen]
impl StackMachine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> StackMachine {
        StackMachine { stack: Vec::new() }
    }

    #[wasm_bindgen]
    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    #[wasm_bindgen]
    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    #[wasm_bindgen]
    pub fn add(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a + b);
    }
    #[wasm_bindgen]
    pub fn sub(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a - b);
    }

    #[wasm_bindgen]
    pub fn peek(&self) -> Option<i32> {
        self.stack.last().cloned()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    // Configure to run tests in the browser
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_stack_machine() {
        let mut stack_machine = StackMachine::new();
        stack_machine.push(10);
        stack_machine.push(20);
        stack_machine.add();
        assert_eq!(stack_machine.peek(), Some(30));
    }

    #[wasm_bindgen_test]
    async fn test_addition_with_insufficient_values() {
        let mut stack_machine = StackMachine::new();
        stack_machine.push(10);
        stack_machine.add();
        assert_eq!(stack_machine.peek(), None);
    }

    #[wasm_bindgen_test]
    async fn test_subtraction_with_insufficient_values() {
        let mut stack_machine = StackMachine::new();
        stack_machine.push(10);
        stack_machine.sub();
        assert_eq!(stack_machine.peek(), None);
    }
}

