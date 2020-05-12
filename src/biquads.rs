///
/// 

pub trait Biquad {
    fn filter(self, v_in: f32, coeffs: BiquadCoeffs) -> f32;
}

pub enum CommonFilters {
    Bandpass,
    Bandstop,
    Lowpass,
    Highpass,
    //TODO: implement shelves
    //Highshelve,
    //Lowshelve,
}

#[derive(PartialEq)]
pub struct BiquadCoeffs {
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
}

impl BiquadCoeffs {
    // just default to zero
    pub fn new() -> BiquadCoeffs {
        BiquadCoeffs {
            a1: 0.0,
            a2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

pub struct DirectFormII {
    //holds state of input and output for delay line
    acc_in: (f32, f32),
    acc_out: (f32, f32),
}

impl DirectFormII {
    pub fn new() -> DirectFormII {
        DirectFormII {
        acc_in: (0.0, 0.0),
        acc_out: (0.0, 0.0),
        }
    }
    fn shift_delayline(mut self, v_in: f32, accum: f32) {
        self.acc_in.1 = self.acc_in.0;
        self.acc_in.0 = v_in;
        self.acc_out.1 = self.acc_out.0;
        self.acc_out.0 = accum;
    }
}

impl Biquad for DirectFormII {
    fn filter(self, v_in: f32, coeffs: BiquadCoeffs) -> f32 {
        let mut accum: f32;
    
        // calculate feed foward 
        accum = (coeffs.b0 * v_in) + (coeffs.b1 * self.acc_in.0) + (coeffs.b2 * self.acc_in.1);
        // calculate feedback 
        accum += -( (coeffs.a1 * self.acc_out.0) + (coeffs.a2 * self.acc_out.1) );
        // Bristow-Johnson extra coefficient (a0) (not used rn, may change) 
        //accum = accum / coeffs.a0;
        
        self.shift_delayline(v_in, accum);
        accum
    }
    
}

// TODO: maybe take optional arg for biquad coefficients so we can clone
// put all these args in struct/enum with generic type arg like in C (so something like Params<T>)
pub fn calculate_coeffs(filt_type: CommonFilters, cutoff_frequency: u32, q_factor: f32, sample_rate: u32) -> BiquadCoeffs {
    use std::f32;
    let s_rate = sample_rate as f32;
    let f_cut = cutoff_frequency as f32;
    match filt_type {
        CommonFilters::Bandpass => {
            let omega: f32 = (2.0 * std::f32::consts::PI * f_cut) / s_rate;
            let alpha: f32 = omega.sin() / (2.0 * q_factor);
            let coeffs: BiquadCoeffs;

            // bandpass, 0db peak gain (from audio-eq-cookbook)
            let mut bq = BiquadCoeffs::new();
            bq.a1 = -2.0 * omega.cos();
            bq.a2 = 1.0 - alpha;
            bq.b0 = alpha;
            bq.b1 = 0.0;
            bq.b2 = -alpha;
            return bq;
        },
        CommonFilters::Lowpass => {
            let omega: f32 = (2.0 * std::f32::consts::PI * f_cut) / s_rate;
            let alpha: f32 = omega.sin() / (2.0 * q_factor);
            let coeffs: BiquadCoeffs;

            // recalculate coefficients using lowpass formula
            let mut bq = BiquadCoeffs::new();
            bq.a1 = -2.0 * omega.cos();
            bq.a2 = 1.0 - alpha;
            bq.b0 = (1.0 - omega.cos()) / 2.0;
            bq.b1 = 1.0 - omega.cos();
            bq.b2 = bq.b0;
            return bq;
        },
        CommonFilters::Highpass => {
            let omega: f32 = (2.0 * std::f32::consts::PI * f_cut) / s_rate;
            let alpha: f32 = omega.sin() / (2.0 * q_factor);
            let coeffs: BiquadCoeffs;

            // recalculate coefficients using highpass formula
            let mut bq = BiquadCoeffs::new();
            bq.a1 = -2.0 * omega.cos();
            bq.a2 = 1.0 - alpha;
            bq.b0 = (1.0 + omega.cos()) / 2.0;
            bq.b1 = -(1.0 + omega.cos());
            bq.b2 = bq.b0;
            return bq;
        },
        _ => {
            //not valid type
            return BiquadCoeffs {
                a1: 0.0,
                a2: 0.0,
                b0: 0.0,
                b1: 0.0,
                b2: 0.0,
            };
            
        }
    }
}

//TODO: seperate test directory structure, just putting here for now
// ---------------------------unit tests---------------------------//
#[test]
fn calculate_coeffs_TEST() {
    let round_2dp = | input: f32 | { (input * 100.00).round() / 100.0 };
    let round_coeffs = | unrounded_coeffs: BiquadCoeffs | {
        let mut coeffs = BiquadCoeffs::new();
        coeffs.a1 = round_2dp(unrounded_coeffs.a1);
        coeffs.a2 = round_2dp(unrounded_coeffs.a2);
        coeffs.b0 = round_2dp(unrounded_coeffs.b0);
        coeffs.b1 = round_2dp(unrounded_coeffs.b1);
        coeffs.b2 = round_2dp(unrounded_coeffs.b2);
        coeffs 
    };
    //can't seem to get partialeq working so doing manual comparison of fields
    let assert_coeffs = | coeffs_a: BiquadCoeffs, coeffs_b: BiquadCoeffs | {
        assert_eq!(coeffs_a.a1, coeffs_b.a1);
        assert_eq!(coeffs_a.a2, coeffs_b.a2);
        assert_eq!(coeffs_a.b0, coeffs_b.b0);
        assert_eq!(coeffs_a.b1, coeffs_b.b1);
        assert_eq!(coeffs_a.b2, coeffs_b.b2);
    };

    // high pass (f0 = 1kHz, fs = 44.1kHz, Q = 0.5)
    {
        let bq: BiquadCoeffs = calculate_coeffs(CommonFilters::Highpass, 1000, 0.5, 44100); 
        //calculated by hand https://www.w3.org/2011/audio/audio-eq-cookbook.html
        let bq_expected = BiquadCoeffs{a1: -1.98, a2: 0.86, b0: 0.99, b1: -1.99, b2: 0.99};
        let rounded_coeffs: BiquadCoeffs = round_coeffs(bq);
        assert_coeffs(bq_expected, rounded_coeffs);
    }

    //low pass (f0 = 10Khz, fs = 96Khz, Q = 1.0)
    {
        let bq: BiquadCoeffs = calculate_coeffs(CommonFilters::Lowpass, 10000, 1.0, 96000); 
        //calculated by hand https://www.w3.org/2011/audio/audio-eq-cookbook.html
        let bq_expected = BiquadCoeffs{a1: -1.59, a2: 0.70, b0: 0.10, b1: 0.21, b2: 0.10};
        let rounded_coeffs: BiquadCoeffs = round_coeffs(bq);
        assert_coeffs(bq_expected, rounded_coeffs);
    }

    //band pass (f0 = 7.5Khz, fs = 48Khz, Q = 3.0)
    {
        let bq: BiquadCoeffs = calculate_coeffs(CommonFilters::Bandpass, 7500, 3.0, 48000); 
        //calculated by hand https://www.w3.org/2011/audio/audio-eq-cookbook.html
        let bq_expected = BiquadCoeffs{a1: -1.11, a2: 0.86, b0: 0.14, b1: 0.0, b2: -0.14};
        let rounded_coeffs: BiquadCoeffs = round_coeffs(bq);
        assert_coeffs(bq_expected, rounded_coeffs);
    }

    //TODO: band pass (f0 = 5Khz, fs = 44.1kHz, Q = 5.0)
    //TODO: high shelve
    //TODO: low shelve
}

#[test]
fn DirectFormII_TEST() {
    //TODO: not sure how best to test, just checking it returns 0 if filtering 0...
    let mut dfii = DirectFormII::new();
    let bq: BiquadCoeffs = calculate_coeffs(CommonFilters::Highpass, 7500, 0.5, 44100);
    assert_eq!(0.0, dfii.filter(0.0, bq));
}

