use crate::tensor::TensorBuffer; 
pub struct ReplayBuffer {
    state: TensorBuffer,
    visit_count_policy: f32,
    value: f32,
}

impl ReplayBuffer {
    pub fn state(&self) -> &TensorBuffer {&self.state}
    pub fn visit_count_policy(&self) -> f32 {self.visit_count_policy}
    pub fn value(&self) -> f32 {self.value}
}