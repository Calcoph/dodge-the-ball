use crate::configuration::RAND_RANGE;
use js_sys;

pub struct Brain {
    pub inputs: u8,
    pub layers: Vec<NeuralLayer>, // TODO remove 'pub'
}

pub struct NeuralLayer { // TODO remove 'pub'
    neurons: Vec<Neuron>
}

struct Neuron {
    weights: Vec<f64>
}

impl Brain {
    /// Generate a brain from scratch
    pub fn new_random(neurons_per_layer: Vec<u8>) -> Brain {
        let mut layers = Vec::new();
        let inputs = neurons_per_layer[0];
        let length = neurons_per_layer.len();
        for i in 0..length {
            let layer =
                if i < length-1 {
                    NeuralLayer::new_random(
                        neurons_per_layer[i],
                        neurons_per_layer[i+1]
                    )
                } else {
                    NeuralLayer::new_random(
                        neurons_per_layer[i],
                        0
                    )
                };

            layers.push(layer);
        }

        Brain {
            layers,
            inputs
        }
    }

    /// Generate a new brain by combining features of both of its parents
    /* pub fn new_mating(brain1: &Brain, brain2: &Brain) -> Brain {
        // TODO
    } */

    pub fn get_output(&self, values: Vec<f64>) -> f64 {
        if values.len() as u8 != self.inputs {panic!("wrong inputs!")}
        let mut output = values;
        for i in &self.layers {
            output = i.get_outputs(&output)
        }

        if output.len() != 1 {panic!("should only have 1 output")}
        output[0]
    }
}

impl Neuron {
    fn new_random(outputs: u8) -> Neuron {
        let mut weights = Vec::new();
        if outputs == 0 {
            // neuron is the output neuron, so it outputs its input
            weights.push(1.0);
        } else {
            for _ in 0..outputs {
                weights.push(random_range(-RAND_RANGE, RAND_RANGE));
            }
        }

        Neuron {
            weights
        }
    }

    fn get_outputs(&self, input: f64) -> Vec<f64>{
        self.weights.iter()
            .map(|i| i*input)
            .collect()
    }
}

impl NeuralLayer {
    fn new_random(neuron_count: u8, next_layer_neuron_count: u8) -> NeuralLayer {
        let mut neurons = Vec::new();
        for _ in 0..neuron_count {
            let neuron = Neuron::new_random(next_layer_neuron_count);
            neurons.push(neuron)
        }

        NeuralLayer {
            neurons
        }
    }

    fn get_outputs(&self, inputs: &Vec<f64>) -> Vec<f64>{
        let mut outputs = self.neurons[0].get_outputs(inputs[0]);
        for i in 1..self.neurons.len() {
            outputs = sum_vectors(outputs, self.neurons[i].get_outputs(inputs[i]))
        }

        outputs
    }
}

fn sum_vectors(vec1: Vec<f64>, vec2: Vec<f64>) -> Vec<f64> {
    vec1.iter()
        .zip(vec2)
        .map(|(i, j)| i+j)
        .collect()
}

fn random_range(min: f64, max: f64) -> f64 {
    js_sys::Math::random() * (max - min) + min
}