///
///
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct GeneratorError {
    details: String
}
impl GeneratorError {
    pub fn new(msg: &str) -> GeneratorError {
        GeneratorError{details: msg.to_string()}
    }
}
impl fmt::Display for GeneratorError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}
impl Error for GeneratorError {
    fn description(&self) -> &str {
        &self.details
    }
}
//----------------------------------------------------------------------------
use crate::biquads::{ 
    DirectFormII, 
    calculate_coeffs, 
    BiquadCoeffs,
    Biquad,
    CommonFilters
};

extern crate rand;
use rand::*; //TODO: what do we actually need...

pub trait Generator {
    fn generate(self) -> f32;
}
#[derive(Debug, PartialEq)]
pub struct Resonator {
    biquad: DirectFormII,
    coeffs: BiquadCoeffs,
    sample_rate: u32,
    q_factor: f32
}

impl Resonator {
    pub fn new() -> Resonator {
        let biquad = DirectFormII::new();
        let mut coeffs: BiquadCoeffs = calculate_coeffs(CommonFilters::Bandpass, 1000.0, 1.0, 44100);
        Resonator {
            biquad: biquad,
            coeffs: coeffs,
            sample_rate: 44100, //should query system sound for this
            q_factor: 1.0
        }
    }
    pub fn set_cutoff(mut self, f_cut: f32) -> Result<(), GeneratorError> {
        let nyquist: f32 = (self.sample_rate as f32) / 2.0;
        if f_cut <= nyquist  {
            //TODO: should take reference to coeffs so we'd only be copying on the way out (lifetimes and aw that jazz)
            Ok(self.coeffs = calculate_coeffs(CommonFilters::Bandpass, f_cut, self.q_factor, self.sample_rate))
        } 
        else {
            Err(GeneratorError::new("[ERROR] requested frequency is out of range"))
        }
    } 
    pub fn set_qfactor(mut self, q_fact: f32) -> Result<(), GeneratorError> {
        //TODO: find decouply way to share common accepted range from engine
        let max_q: f32 = 20.0;  
        if (0.0..max_q).contains(&q_fact) {
            self.q_factor = q_fact;
            Ok(())
        }
        else {
            Err(GeneratorError::new("[ERROR] Q value out of range"))
        }
    } 
    pub fn set_samplerate(mut self, s_rate: u32) -> Result<(), GeneratorError> {
        //TODO: same as set_q
        let max_srate: u32 = 196000; //has to be placeholder, if goes wrong audio hardware could crash
        if (0..max_srate).contains(&s_rate) {
            self.sample_rate = s_rate;
            Ok(())
        }
        else {
            Err(GeneratorError::new("[ERROR] sample rate out of range"))
        }
    }
    
}

///
/// generates basic noise and applies the subtractive filter/bandpass
impl Generator for Resonator {
    fn generate(self) -> f32 {
        let mut rng = rand::thread_rng();
        let noise_sample: f32 = rng.gen_range(-1.0, 1.0);
        self.biquad.filter(noise_sample, self.coeffs)
    }
}

impl Copy for Resonator { }
impl Clone for Resonator {
    fn clone(&self) -> Resonator {
        *self
    }
}
//----------------------------------------------------------------------
use more_asserts::*;

#[test]
#[allow(non_snake_case)]
fn resonator_set_cutoff_test_PASS() {
    let res = Resonator::new();
    let result = res.set_cutoff(14000.0); //<nyquist at s_rate = 44.1kHz
    assert_eq!(result, Ok(()));
}

#[test]
#[allow(non_snake_case)]
fn resonator_set_cutoff_test_FAIL() {
    let res = Resonator::new();
    let result = res.set_cutoff(30000.0);//> nyquist at s_rate = 44.1kHz
    assert_eq!(result, Err(GeneratorError::new("[ERROR] requested frequency is out of range")));
}

#[allow(non_snake_case)]
#[test]
fn resonator_set_qfactor_test_PASS() {
    let res = Resonator::new();
    let result = res.set_qfactor(8.8);
    assert_eq!(result, Ok(()));
}

#[allow(non_snake_case)]
#[test]
fn resonator_set_qfactor_test_FAIL() {
    let res = Resonator::new();
    let result = res.set_qfactor(21.0);
    assert_eq!(result, Err(GeneratorError::new("[ERROR] Q value out of range")));

}

#[allow(non_snake_case)]
#[test]
fn resonator_set_samplerate_test_PASS() {
    let res = Resonator::new();
    let result = res.set_samplerate(48000); 
    assert_eq!(result, Ok(()));
}

#[allow(non_snake_case)]
#[test]
fn resonator_set_samplerate_test_FAIL() {
    let res = Resonator::new();
    let result = res.set_samplerate(300000);
    assert_eq!(result, Err(GeneratorError::new("[ERROR] sample rate out of range")));
}
// TODO: refactor
#[allow(non_snake_case)]
#[test]
fn resonator_generate_PASS() {
    use itertools::*;
    let res = Resonator::new();
    let mut test_buffer: [f32; 256] = [1.0; 256];
    for i in 0..256 {
        test_buffer[i] = res.generate();
    } 
    //check min and max bounds
    assert_ge!(1.0, test_buffer.iter().cloned().fold1(f32::max).unwrap());
    assert_le!(-1.0, test_buffer.iter().cloned().fold1(f32::max).unwrap()); 
    assert_eq!(false, test_buffer.is_empty()); //check it's not just all zeros
    //for i in 0..256 { print!("{:?}", test_buffer[i]); } //dump buffer in debug mode 
}


