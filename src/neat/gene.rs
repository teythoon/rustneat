extern crate rand;

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Gene {
    in_neuron_id: usize,
    out_neuron_id: usize,
    weight: f64,
    enabled: bool,
    innovation: u64,
}

impl Eq for Gene {}

impl PartialEq for Gene {
    fn eq(&self, other: &Gene) -> bool {
        self.in_neuron_id == other.in_neuron_id && self.out_neuron_id == other.out_neuron_id
    }
}

impl Ord for Gene {
    fn cmp(&self, other: &Gene) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.in_neuron_id == other.in_neuron_id {
            if self.out_neuron_id > other.out_neuron_id {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if self.in_neuron_id > other.in_neuron_id {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Gene {
    fn partial_cmp(&self, other: &Gene) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Gene {
    pub fn generate_weight() -> f64 {
        rand::random::<f64>() * 2f64 - 1f64
    }
}

impl Default for Gene {
    fn default() -> Gene {
        Gene {
            in_neuron_id: 1,
            out_neuron_id: 1,
            weight: Gene::generate_weight(),
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use neat::*;

    fn g(n_in: usize, n_out: usize) -> Gene {
        Gene {
            in_neuron_id: n_in,
            out_neuron_id: n_out,
            ..Gene::default()
        }
    }

    #[test]
    fn should_be_able_to_binary_search_for_a_gene() {
        let mut genome = vec![g(0, 1), g(0, 2), g(3, 2), g(2, 3), g(1, 5)];
        genome.sort();
        genome.binary_search(&g(0, 1)).unwrap();
        genome.binary_search(&g(0, 2)).unwrap();
        genome.binary_search(&g(1, 5)).unwrap();
        genome.binary_search(&g(2, 3)).unwrap();
        genome.binary_search(&g(3, 2)).unwrap();
    }
}
