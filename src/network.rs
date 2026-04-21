/*#[derive(Debug)]
pub struct Network{
    layers:Ver<Layer>
}

impl Network {
    pub fn propagate(&self,inputs:Vec<f32>) -> Vec<f32> {
        let mut inputs = inputs;
        for layer in &self.layers {
            inputs = layer.propage(inputs);
        }
        inputs
    }
}

struct layer{
    neurons:Vec<Neuron>
}

impl Layer {
    fn propagate(&self,inputs:Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();
        for neurons in &self.neurons{
            let outputs = neuron.propagate(&inputs);
            outputs.push(outputs);
        }
        outputs
    }
}


#[derive(Debug)]
pub struct LayerTopology {
    pub input_neurons: usize,
    pub output_neurons: usize,
}


struct Neuron {
    bais:f32,
    weights:Vec<f32>
}


impl Neuron {
   /*  fn propagate(&self,inputs:&[f32]) -> f32 {
        let mut outputs = 0.0;
        for i in 0..inputs.len(){
            output += inputs[i] * self.weights[1];
        }
        output += self.bais;
        if output > 0.0 {
            output
        } else {
            0.0
        }
    }*/

    fn propagate(&self,inputs:&[f32]) -> f32 {
        assert_eq!(inputs.len(),self.weights.len());
       let output = inputs
       .iter()
       .zip(&self.weights)
       .map(|(inputs,weights) | input * weight)
       .sum::<f32>();
    (self.bias + output).max(0.0)
    }

    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

}*/

/*use rand::Rng;


#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let sum: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum();

        (self.bias + sum).max(0.0)
    }

    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_propagate() {
        let neuron = Neuron {
            bias: 0.0,
            weights: vec![0.5, 0.5, 0.5],
        };

        let inputs = vec![1.0, 1.0, 1.0];

        let output = neuron.propagate(&inputs);

        assert_eq!(output, 1.5);
    }

    #[test]
    fn test_layer_propagate() {
        let layer = Layer {
            neurons: vec![
                Neuron {
                    bias: 0.0,
                    weights: vec![1.0, 1.0],
                },
                Neuron {
                    bias: 0.0,
                    weights: vec![0.5, 0.5],
                },
            ],
        };

        let inputs = vec![1.0, 1.0];

        let outputs = layer.propagate(inputs);

        assert_eq!(outputs, vec![2.0, 1.0]);
    }

    #[test]
    fn test_network_propagate() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![
                        Neuron {
                            bias: 0.0,
                            weights: vec![1.0, 1.0],
                        },
                        Neuron {
                            bias: 0.0,
                            weights: vec![0.5, 0.5],
                        },
                    ],
                },
                Layer {
                    neurons: vec![
                        Neuron {
                            bias: 0.0,
                            weights: vec![1.0, 1.0],
                        },
                    ],
                },
            ],
        };

        let inputs = vec![1.0, 1.0];

        let outputs = network.propagate(inputs);

        assert_eq!(outputs, vec![3.0]);
    }
}*/

use rand::{Rng, RngCore};

// ------------------------------------------------
// Public API
// ------------------------------------------------

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

// ------------------------------------------------
// Private internals
// ------------------------------------------------

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

// ------------------------------------------------
// Tests
// ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn neuron_random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
    }

    #[test]
    fn neuron_propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Tests ReLU (negative output should be clamped to 0.0)
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);

        // Tests normal forward pass: (-0.3 * 0.5) + (0.8 * 1.0) + 0.5 = 1.15
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }
}