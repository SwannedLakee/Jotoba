use ngram_tools::iter::wordgrams::Wordgrams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vsm::Vector;

#[derive(Serialize, Deserialize)]
pub struct NgIndex {
    freq: HashMap<String, u32>,
    ng_ids: HashMap<String, u32>,
    n: usize,
    total: usize,
}

impl NgIndex {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        Self {
            freq: HashMap::new(),
            ng_ids: HashMap::new(),
            n,
            total: 0,
        }
    }

    /// Returns the amount of indexed terms
    #[inline]
    pub fn len(&self) -> usize {
        self.freq.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn index_new(&mut self, gloss: &str) {
        let padded = self.get_paddded(gloss);
        let n = self.n_for(gloss);
        let ngrams = Wordgrams::new(&padded, n);

        for ngram in ngrams {
            self.insert(ngram.to_string());
        }
    }

    pub fn insert(&mut self, ng: String) {
        self.total += 1;
        if let Some(freq) = self.freq.get_mut(&ng) {
            *freq += 1;
            return;
        }

        let new_id = self.ng_ids.len() as u32;
        self.ng_ids.insert(ng.clone(), new_id);
        self.freq.insert(ng.clone(), 1);
    }

    pub fn compress(&mut self, threshold: usize) {
        // Remove all terms wit frequency `threshold` and treat out of dict
        // ngrams as 1 to save memory
        self.freq.retain(|k, v| {
            if *v < threshold as u32 {
                self.ng_ids.remove(k).unwrap();
                return false;
            }

            true
        });
    }

    #[inline]
    pub fn freq(&self, term: &str) -> f32 {
        self.freq.get(term).map(|i| *i as f32).unwrap_or(1.0)
    }

    #[inline]
    pub fn inverted_freq(&self, term: &str) -> f32 {
        let freq = self.freq(term);
        let total = self.total as f32;
        (total / freq).log(1.1)
    }

    #[inline]
    pub fn sim<A, B>(&self, a: A, b: B) -> f32
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let a_vec = self.build_vec(a);
        let b_vec = self.build_vec(b);

        let (a_vec, b_vec) = match (a_vec, b_vec) {
            (Some(a), Some(b)) => (a, b),
            _ => return 0.0,
        };

        sim(&a_vec, &b_vec)
    }

    pub fn build_vec<A: AsRef<str>>(&self, inp: A) -> Option<Vector> {
        let inp = inp.as_ref();
        let padded = self.get_paddded(inp);
        let n = self.n_for(inp);

        let ng_ids: Vec<_> = Wordgrams::new(&padded, n)
            .map(|i| {
                let id = self.ng_ids.get(i).copied().unwrap_or(0);
                let freq = self.inverted_freq(i);
                (id, freq as f32)
            })
            .collect();

        Some(Vector::create_new_raw(ng_ids))
    }

    #[inline]
    fn n_for(&self, inp: &str) -> usize {
        self.n.min(inp.len())
    }

    #[inline]
    fn get_paddded(&self, inp: &str) -> String {
        let n = self.n_for(inp);
        ngram_tools::padding(inp, n - 1)
    }
}

#[inline]
fn sim(a: &Vector, b: &Vector) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let both = a.overlapping(b).map(|(_, a_w, b_w)| a_w + b_w).sum::<f32>();

    let sum = a
        .sparse()
        .iter()
        .map(|i| i.1)
        .chain(b.sparse().iter().map(|i| i.1))
        .sum::<f32>();

    both / sum
}
