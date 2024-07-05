pub mod combine_streams;
pub mod audio_stream; 
pub mod modulation_stream;

#[derive(Debug)]
pub enum CommandInterThread {
    GetStatus,
    StatusActive,
    StatusSuspended,
    ResumeAudioStream,
    TerminateYourSelf,
    SuspendAudioStream,
    ErrorStartingKillingSelf,
}

#[derive(Debug, Clone)]
pub enum ThreadType {
    InputCaptrureThread,
    OutputCaptureThread,
    ApplicationCaptureThread,
}
