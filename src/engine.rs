///
/// 

pub mod engine {

    extern crate rb; //safe static allocated buffer https://github.com/klingtnet/rb
    use rb::*;

    pub struct AudioEngine {
        buffer_size: usize,
        sample_rate: i32,
        output_buffer: SpscRb<f32>, //a single producer single consumer buffer for our output
        //resonator

        
    }

    impl AudioEngine {
        fn new(buffer_size: usize, sample_rate: i32) -> AudioEngine {
            let output_buffer = SpscRb::new(buffer_size); 
            AudioEngine {
                buffer_size: buffer_size,
                sample_rate: sample_rate,
                output_buffer: output_buffer
            }
        }
        // checks config and returns a suitable buf-size and sr that meets standard
        pub fn validate_audio_config(buffer_size: usize, sample_rate: i32) -> (usize, i32) {
            // chunk size should be power of two
            let mut temp_val = buffer_size;
            while temp_val%2 == 0 {
                temp_val /= 2;
            }
            // 2048 seems reasonable to cover most modern machines/audio backends
            let mut ret_bsize = if temp_val == 1 { buffer_size } else { 2048 };
            let mut ret_srate: i32;
            match sample_rate {
                //standard sample rates are acceptable
                441000 | 480000 | 960000 => ret_srate = sample_rate,
                //default to 44.1kHz, most common
                _ => ret_srate = 441000,
            }
            (ret_bsize, ret_srate)
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
    fn new_TEST() {
        let buf_size: usize = 1024;
        let s_rate: i32 = 441000;
        let engine = AudioEngine::new(buf_size, s_rate);
        assert_eq!(buf_size, engine.buffer_size);
        assert_eq!(s_rate, engine.sample_rate);
    }

    #[test]
    fn validate_audio_config_TEST() {
        assert_eq!(AudioEngine::validate_audio_config(1024, 441000), (1024, 441000));
        assert_ne!(AudioEngine::validate_audio_config(2048, 960000), (1024, 441000));
        assert_eq!(AudioEngine::validate_audio_config(4096, 960000), (4096, 960000));
    }
    

}

