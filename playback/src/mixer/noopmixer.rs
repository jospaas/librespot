use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::VolumeGetter;
use super::{MappedCtrl, VolumeCtrl};
use super::{Mixer, MixerConfig};

#[derive(Clone)]
pub struct NoOpMixer {
    // There is no AtomicF64, so we store the f64 as bits in a u64 field.
    // It's much faster than a Mutex<f64>.
    volume: Arc<AtomicU64>,
    volume_ctrl: VolumeCtrl,
}

impl Mixer for NoOpMixer {
    fn open(config: MixerConfig) -> Result<Self, Error>  {
        let volume_ctrl = config.volume_ctrl;
        info!("Mixing with NoOp mixer");

        Ok(Self {
            volume: Arc::new(AtomicU64::new(f64::to_bits(0.5))),
            volume_ctrl,
        })
    }

    fn volume(&self) -> u16 {
        let mapped_volume = f64::from_bits(self.volume.load(Ordering::Relaxed));
        self.volume_ctrl.as_unmapped(mapped_volume)
    }

    fn set_volume(&self, volume: u16) {
        let mapped_volume = self.volume_ctrl.to_mapped(volume);
        self.volume
            .store(mapped_volume.to_bits(), Ordering::Relaxed)
    }
}

impl NoOpMixer {
    pub const NAME: &'static str = "noop";
}

struct NoOpVolume(Arc<AtomicU64>);

impl VolumeGetter for NoOpVolume {
    fn attenuation_factor(&self) -> f64 {
        f64::from_bits(self.0.load(Ordering::Relaxed))
    }
}
