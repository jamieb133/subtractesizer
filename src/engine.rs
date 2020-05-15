///
/// 
extern crate rb; //safe static allocated buffer https://github.com/klingtnet/rb
use rb::*;

use crate::generators::*;
use more_asserts::*;
use std::error::Error;
use std::fmt;
use std::result::Result;

pub struct AudioEngine {
    buffer_size: usize,
    sample_rate: u32,
    output_buffer: SpscRb<f32>, //a single producer single consumer buffer for our output
    resonator: Resonator,
    midi_lookup: [f32; 88] //midi piano notes go from 22 (A0) to  108 (C8)
}

impl AudioEngine {
    pub fn new(buffer_size: usize, sample_rate: u32) -> AudioEngine {
        let output_buffer = SpscRb::new(buffer_size); 
        let resonator = Resonator::new();
        let mut midi_lookup: [f32; 88] = [0.0; 88];
        //precompute frequency values for midi
        for m in 21..109 {
            let m32 = m as f32;
            midi_lookup[m - 21] = 2f32.powf((m32 - 69.0) / 12.0 ) * 440.0;
            println!("{0}", midi_lookup[m-21]);
            //println!("{0}", 2f32.powf((m32 - 21.0 - 69.0) / 12.0 ));
        }


        AudioEngine {
            buffer_size: buffer_size,
            sample_rate: sample_rate,
            output_buffer: output_buffer,
            resonator: resonator,
            midi_lookup: midi_lookup
        }
    }
    // checks config and returns a suitable buf-size and sr that meets standard
    pub fn validate_audio_config(buffer_size: usize, sample_rate: u32) -> (usize, u32) {
        // chunk size should be power of two
        let mut temp_val = buffer_size;
        while temp_val%2 == 0 {
            temp_val /= 2;
        }
        // 2048 seems reasonable to cover most modern machines/audio backends
        let ret_bsize = if temp_val == 1 { buffer_size } else { 2048 };
        let ret_srate: u32;
        match sample_rate {
            //standard sample rates are acceptable
            441000 | 480000 | 960000 => ret_srate = sample_rate,
            //default to 44.1kHz, most common
            _ => ret_srate = 441000,
        }
        (ret_bsize, ret_srate)
    }
    
    //TODO: refactor, this struct future contain multiple fx, implement a request queue
    pub fn set_noteon(&self, midi_val: u32) -> Result<(), GeneratorError> { //TODO: make engine error
        //TODO: mutex
        let f_cut: f32 = self.midi_to_frequency(midi_val);
        /*
        match self.resonator.set_cutoff(f_cut) {
            Ok(()) => Ok(()),
            Err(e) => Err(e), 
        }
        */
        Ok(())
    }

    pub fn set_qfactor(self, q_fact: f32) -> Result<(), GeneratorError> {
        //TODO: mutex
        match self.resonator.set_qfactor(q_fact) {
            Ok(()) => Ok(()),
            Err(e) => Err(e), 
        }
    }

    pub fn set_samplerate(mut self, s_rate: u32) -> Result<(), GeneratorError> {
        //TODO: mutex
        let (_, srate_ret) = AudioEngine::validate_audio_config(self.buffer_size, s_rate);
        if srate_ret == s_rate { 
            self.sample_rate = s_rate;
            match self.resonator.set_samplerate(s_rate) {
                Ok(()) => Ok(()),
                Err(e) => Err(e), 
            }
        }
        else {
            Err(GeneratorError::new("[ERROR] sampling frequency is not valid"))
        }
    }
    
    fn midi_to_frequency(&self, midi_val: u32) -> f32 {
        //convert offset
        let index: usize = midi_val as usize;
        self.midi_lookup[index - 21]
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        println!("Destroying audio engine...")
    }
}

/* TODO
trait RoutesSignal {
    component_list: Vec<AudioFxComponent>,
    fn add_component(...); 
    fn remove_component(...);
}
impl RoutesSignal for AudioEngine {
    ...
}
*/

// ---------------------------unit tests---------------------------//
#[test]
fn audio_engine_new_test() {
    let buf_size: usize = 1024;
    let s_rate: u32 = 441000;
    let engine = AudioEngine::new(buf_size, s_rate);
    assert_eq!(buf_size, engine.buffer_size);
    assert_eq!(s_rate, engine.sample_rate);
}

#[test]
fn audio_engine_validate_audio_config_test() {
    assert_eq!(AudioEngine::validate_audio_config(1024, 441000), (1024, 441000));
    assert_ne!(AudioEngine::validate_audio_config(2048, 960000), (1024, 441000));
    assert_eq!(AudioEngine::validate_audio_config(4096, 960000), (4096, 960000));
}

#[test]
fn audio_engine_midi_to_frequency_test() {
    let engine = AudioEngine::new(1024, 44100);
    assert_eq!(engine.midi_to_frequency(21).round(), 28.0);
    assert_eq!(engine.midi_to_frequency(60).round(), 262.0);
    assert_eq!(engine.midi_to_frequency(69).round(), 440.0);
    assert_eq!(engine.midi_to_frequency(108).round(), 4186.0);
}
