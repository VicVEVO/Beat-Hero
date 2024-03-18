use rodio::{source::SineWave, OutputStream, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SineWave::new(1000).take_duration(std::time::Duration::from_secs(2));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
}
