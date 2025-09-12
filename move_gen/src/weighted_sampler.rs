use core::f32;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct WeightedSampler<T> {
    items: Vec<(T, f32)>,
    total_weight: f32,
}

impl<T: Clone> WeightedSampler<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            total_weight: 0.0,
        }
    }

    pub fn new_from_items(items: Vec<(T, f32)>) -> Self {
        let total_weight = items.iter().map(|(_, w)| w).sum(); 
        Self {
            items,
            total_weight,
        }
    }

    pub fn new_from_vecs(items: Vec<T>, weights: Vec<f32>) -> Result<Self, &'static str> {
        if items.len() != weights.len() {
            return Err("items and weights must be same length")
        }
        let paired = items.into_iter().zip(weights.into_iter()).collect(); 
        Ok(Self::new_from_items(paired))
    }

    pub fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Option<T> {
        if self.items.is_empty()|| self.total_weight <= 0.0 {
            return None
        }
        let target = self.total_weight * rng.r#gen::<f32>(); 
        let mut cumulative = 0.0f32;

        for (i, (_, weight)) in self.items.iter().enumerate() {
            cumulative += weight; 
            if cumulative >= target {
                self.total_weight -= weight; 
                return Some(self.items.swap_remove(i).0); 
            }
        }
        None
    }

    pub fn push(&mut self, item: (T, f32)) {
        self.total_weight += item.1; 
        self.items.push(item); 
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}