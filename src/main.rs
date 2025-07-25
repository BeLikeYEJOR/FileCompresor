use fon::{stereo::Stereo32, Audio, Sink};
use pasts::wait;
use wavy::{Microphone, MicrophoneStream, Speakers, SpeakersSink};

/// An event handled by the event loop.
enum Event<'a> {
    /// Speaker is ready to play more audio.
    Play(SpeakersSink<'a, Stereo32>),
    /// Microphone has recorded some audio.
    Record(MicrophoneStream<'a, Stereo32>),
}

/// Shared state between tasks on the thread.
struct State {
    /// Temporary buffer for holding real-time audio samples.
    buffer: Audio<Stereo32>,
}

impl State {
    fn event(&mut self, event: Event<'_>) {
        println!("RECEIVED EVENT. BUFFER LEN: {}", self.buffer.len());

        match event {
            // Event::Play(mut speakers) => speakers.stream(self.buffer.drain()),
            Event::Record(microphone) => self.buffer.extend(microphone),
            Event::Play(mut speakers) => {
                if !self.buffer.is_empty() {
                    speakers.stream(self.buffer.drain());
                }
            }
        }
    }
}

async fn run(mut state: State, mut mic: Microphone, mut spk: Speakers) {
    loop {
        println!("WAITING FOR AUDIO EVENT...");
        let event = wait! {
            Event::Record(mic.record().await),
            Event::Play(spk.play().await),
        };
        println!("EVENT TRIGGERED");
        state.event(event);
    }
}

fn main() {
    let state = State {
        buffer: Audio::with_silence(48_000, 2),
    };
    let microphone = Microphone::default();
    let speakers = Speakers::default();
    println!("Available microphones: {:?}", Microphone::query());
    println!("Available speakers: {:?}", Speakers::query());

    futures_lite::future::block_on(run(state, microphone, speakers));
}
