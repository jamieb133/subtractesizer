///
/// 

pub mod biquads {

    trait Biquad {
        fn filter(v_in: f32, coeffs: BiquadCoeffs) -> f32;
    }
    
    enum CommonFilters {
        Bandpass,
        Bandstop,
        Lowpass,
        Highpass,
        //TODO: implement shelves
        //Highshelve,
        //Lowshelve,
    }

    struct BiquadCoeffs {
        a1: f32,
        a2: f32,
        b0: f32,
        b1: f32,
        b2: f32,
    }

    impl BiquadCoeffs {
        // just default to zero
        fn new() -> BiquadCoeffs {
            BiquadCoeffs {
                a1: 0.0,
                a2: 0.0,
                b0: 0.0,
                b1: 0.0,
                b2: 0.0,
            }
        }
    }

    struct DirectFormII {
        //holds state of input and output for delay line
        acc_in: f32,
        acc_out: f32, 
    }

    impl Biquad for DirectFormII {
        fn filter(v_32: f32, coeffs: BiquadCoeffs) -> f32 {
            return 0.0; //placeholder
        }
        
    }

    fn calculate_coeffs(filt_type: CommonFilters, cutoff_frequency: i32, q_factor: i32, sample_rate: i32) -> BiquadCoeffs {
        use std::f32;
        let s_rate = sample_rate as f32;
        let f_cut = cutoff_frequency as f32;
        let q_fact = q_factor as f32;
        match filt_type {
            CommonFilters::Bandpass => {
                let omega: f32 = (2.0 * std::f32::consts::PI * f_cut) / s_rate;
                let alpha: f32 = omega.sin() / (2.0 * q_fact);
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
                let alpha: f32 = omega.sin() / (2.0 * q_fact);
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
                let alpha: f32 = omega.sin() / (2.0 * q_fact);
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
}