use crate::tensor::TensorBuffer; 
use rand::seq::SliceRandom; 
use rand::thread_rng; 

pub struct ReplayBufferEntry {
    state: TensorBuffer,
    visit_count_policy: f32,
    value: f32,
}

impl ReplayBufferEntry {
    pub fn state(&self) -> &TensorBuffer {&self.state}
    pub fn visit_count_policy(&self) -> f32 {self.visit_count_policy}
    pub fn value(&self) -> f32 {self.value}

}

pub struct ReplayBuffer {
    buf: Vec<ReplayBufferEntry>,
}

impl ReplayBuffer {
    pub fn append(&mut self, entry: ReplayBufferEntry) {
        self.buf.push(entry); 
    }

    pub fn sample(&self) -> Option<&ReplayBufferEntry> {
        let mut rng = thread_rng(); 
        let sample = self.buf.choose(&mut rng);
        sample 
    }
}