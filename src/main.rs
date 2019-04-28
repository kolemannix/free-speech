use std::path::Path;
use std::env::args;
use std::fs::File;

use deepspeech::Model;
use audrey::read::Reader;
use audrey::sample::interpolate::{Converter, Linear};
use audrey::sample::signal::{from_iter, Signal};
use portaudio::PortAudio;

// These constants are taken from the C++ sources of the client.

const N_CEP: u16 = 26;
const N_CONTEXT: u16 = 9;
const BEAM_WIDTH: u16 = 500;

const LM_WEIGHT: f32 = 0.75;
const VALID_WORD_COUNT_WEIGHT: f32 = 1.85;

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE :u32 = 16_000;

fn main() {


    speech::run().unwrap();

    // let audio_file_path = "/Users/kolemannix/personal/speech/test345.wav";
    // let mut model = DsModel::new();
    // for _ in 1 .. 10 {
    //     let audio_file = File::open(audio_file_path).unwrap();
    //     let reader = Reader::new(audio_file).unwrap();
    //     let time = std::time::SystemTime::now();
    //     let result = model.process_audio(reader);
    //     println!("Took {}ms", time.elapsed().unwrap().as_millis());
    //     println!("{}", result);
    // }
}

struct DsModel {
    model: Model
}
impl DsModel {
    pub fn new() -> DsModel {
        let model_dir_str = "/Users/kolemannix/personal/speech/models";
        let dir_path = Path::new(&model_dir_str);
        let mut model = Model::load_from_files(
	    &dir_path.join("output_graph.pb"),
	    N_CEP,
	    N_CONTEXT,
	    &dir_path.join("alphabet.txt"),
	    BEAM_WIDTH).unwrap();
        model.enable_decoder_with_lm(
	    &dir_path.join("alphabet.txt"),
	    &dir_path.join("lm.binary"),
	    &dir_path.join("trie"),
	    LM_WEIGHT,
	    VALID_WORD_COUNT_WEIGHT);
        DsModel { model }
    }

    pub fn process_audio<R: std::io::Read + std::io::Seek>(&mut self, mut reader: Reader<R>) -> String {
        let desc = reader.description();
        assert_eq!(1, desc.channel_count(),
	           "The channel count is required to be one, at least for now");

        // Obtain the buffer of samples
        let audio_buf :Vec<_> = if desc.sample_rate() == SAMPLE_RATE {
	    reader.samples().map(|s| s.unwrap()).collect()
        } else {
	    // We need to interpolate to the target sample rate
	    let interpolator = Linear::new([0i16], [0]);
	    let conv = Converter::from_hz_to_hz(
	        from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
	        interpolator,
	        desc.sample_rate() as f64,
	        SAMPLE_RATE as f64);
	    conv.until_exhausted().map(|v| v[0]).collect()
        };

        // Run the speech to text algorithm
        let result = self.model.speech_to_text(&audio_buf, SAMPLE_RATE).unwrap();

        result
    }

}
