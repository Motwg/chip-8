use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, source};

pub struct Speaker {
    player: Player,
    _sink: MixerDeviceSink,
}

impl Speaker {
    pub fn new(volume: f32) -> Self {
        let sink = DeviceSinkBuilder::open_default_sink().expect("Open default audio stream.");
        let player = Player::connect_new(&sink.mixer());
        player.set_volume(volume);
        Speaker {
            player,
            _sink: sink,
        }
    }

    pub fn set_sin(&self, freq: f32) {
        self.player.append(source::SineWave::new(freq));
    }

    pub fn play_while(&self, condition: bool) {
        if condition {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn stop(&self) {
        self.player.pause();
    }
}

impl Default for Speaker {
    fn default() -> Self {
        Self::new(1.0)
    }
}
