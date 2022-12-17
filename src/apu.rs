pub struct Apu {}

impl Apu {
    // Call emulator.step() from the audio callback (or audio thread?).
    // Run it for the number of cycles needed to generate the needed number of
    // samples. I think an improvement on this is using a fifo queue for
    // audio samples. Block the emulator from running when the queue is full.
    // So syncing to audio is basically blocking the emulator on audio output (I think)

    // So cpal will have an audio callback. The callback will give a buffer to
    // fill. Run the emulator in the callback. Step enough times to fill the buffer.
    fn write(&mut self, data: u8) {
        todo!()
    }
}
