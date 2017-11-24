extern crate rodio;

fn main() {
    use std::fs::File;
    use std::io::BufReader;

    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open("sound.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
