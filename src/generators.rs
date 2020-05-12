///
///
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct GeneratorError {
    details: String
}
impl GeneratorError {
    fn new(msg: &str) -> GeneratorError {
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

trait Generator {
    fn generate(self) -> f32;
}

struct Resonator {
    biquad: DirectFormII,
    coeffs: BiquadCoeffs,
    sample_rate: u32,
    q_factor: f32
}

impl Resonator {
    fn new(self) -> Resonator {
        let biquad = DirectFormII::new();
        let coeffs = BiquadCoeffs::new();
        Resonator {
            biquad: biquad,
            coeffs: coeffs,
            sample_rate: 44100, //should query system sound for this
            q_factor: 0.0
        }
    }
    fn set_cutoff(mut self, f_cut: u32) -> Result<(), GeneratorError> {
        let nyquist: u32 = self.sample_rate / 2;
        if (0..nyquist).contains(&f_cut)  {
            //TODO: should take reference to coeffs so we'd only be copying on the way out (lifetimes and aw that jazz)
            Ok(self.coeffs = calculate_coeffs(CommonFilters::Bandpass, f_cut, self.q_factor, self.sample_rate))
        } 
        else {
            Err(GeneratorError::new("[ERROR] requested frequency is out of range"))
        }
    } 
    fn set_qfactor(mut self, q_fact: f32) -> Result<(), GeneratorError> {
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
    fn set_samplerate(mut self, s_rate: u32) -> Result<(), GeneratorError> {
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
        let noise_sample: f32 = rng.gen();
        self.biquad.filter(noise_sample, self.coeffs)
    }
}
