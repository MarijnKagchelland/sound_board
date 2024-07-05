use sound_board::*;

fn main() {
//    let (mut help0, _, com_rx0, com_tx0) = AudioStream::application_template(4096, 11111);
//    let _handle = thread::Builder::new()
//        .name("help0".to_string())
//        .spawn(move || help0.audio_thread());
//    for _ in 0..10000 {
//
//    }
//    let _ = com_tx0.send(CommandInterThread::TerminateYourSelf);
//    sleep(Duration::from_secs(2));
//    let mut help0_bool = true;
//    while help0_bool {
//        match com_tx0.send(CommandInterThread::Ping) {
//            Ok(_)       => (),
//            Err(err)    => {
//                debug!("Err: {}", err);
//                help0_bool = false;
//            }
//        }
//    }
    AudioMixerApp::start();
}
