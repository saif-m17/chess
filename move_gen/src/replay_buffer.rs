use crate::tensor::TensorBuffer; 
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom; 
use rand::thread_rng; 

pub struct ReplayBufferEntry {
    state: TensorBuffer,
    visit_count_policy: Vec<f32>,
    value: f32,
}

impl ReplayBufferEntry {
    pub fn state(&self) -> &TensorBuffer {&self.state}
    pub fn visit_count_policy(&self) -> &Vec<f32> {&self.visit_count_policy}
    pub fn value(&self) -> f32 {self.value}

    pub fn new(state: TensorBuffer, visit_count_policy: Vec<f32>, value: f32) -> Self {
        Self {
            state, visit_count_policy, value,
        }
    }
}

pub struct ReplayBuffer {
    buf: Vec<ReplayBufferEntry>,
    rng: ThreadRng,
}

impl ReplayBuffer {
    pub fn new() -> Self {
        ReplayBuffer { buf: Vec::new(), rng: thread_rng() }
    }
    pub fn append(&mut self, entry: ReplayBufferEntry) {
        self.buf.push(entry); 
    }

    pub fn sample(&mut self) -> Option<&ReplayBufferEntry> {
        let sample = self.buf.choose(&mut self.rng);
        sample 
    }

    pub fn sample_batch(&mut self, batch_size: usize) -> Vec<&ReplayBufferEntry> {
        self.buf.choose_multiple(&mut self.rng, batch_size).collect()
    }
}