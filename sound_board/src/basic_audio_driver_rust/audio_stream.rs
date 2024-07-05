use crate::log_system::*;
use crate::basic_audio_driver_rust::{ThreadType, CommandInterThread};

use std::sync::mpsc;
use std::error::{self};
use std::usize;
use std::vec::Vec;
use std::collections::VecDeque;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub struct AudioStream {
    audio_block_size:   usize,
    audio_channel_tx:   Option<mpsc::Sender<Vec<u8>>>,
    audio_channel_rx:   Option<mpsc::Receiver<Vec<u8>>>,
    communication_tx:   mpsc::Sender<CommandInterThread>,
    communication_rx:   mpsc::Receiver<CommandInterThread>,
    process_id:         Option<u32>,
    thread_type:        ThreadType,
    thread_active:      bool,
    name:               String
}

impl AudioStream {
    pub fn application_template(block_size: usize, pid: u32, name: &str) -> (
        Self, 
        mpsc::Receiver<Vec<u8>>,
        mpsc::Receiver<CommandInterThread>, 
        mpsc::Sender<CommandInterThread>) 
    {
        let (audio_tx, audio_rx) = mpsc::channel();
        let (com_tx, com_thread_rx) = mpsc::channel();
        let (com_thread_tx, com_rx) = mpsc::channel();
        let name_app = String::from(name);
        return (Self {
            audio_block_size:   block_size,
            audio_channel_rx:   None,
            audio_channel_tx:   Some(audio_tx),
            communication_tx:   com_thread_tx,
            communication_rx:   com_thread_rx,
            process_id:         Some(pid),
            thread_type:        ThreadType::ApplicationCaptureThread,
            thread_active:      false,
            name:               name_app
        }, audio_rx, com_rx, com_tx)
    }

    pub fn output_hardware_template(block_size: usize, name: &str) -> (
        Self, 
        mpsc::Sender<Vec<u8>>,
        mpsc::Receiver<CommandInterThread>, 
        mpsc::Sender<CommandInterThread>) 
    {
        let (audio_tx, audio_rx) = mpsc::channel();
        let (com_tx, com_thread_rx) = mpsc::channel();
        let (com_thread_tx, com_rx) = mpsc::channel();
        let name_output = String::from(name);
        return (Self {
            audio_block_size:   block_size,
            audio_channel_rx:   Some(audio_rx),
            audio_channel_tx:   None,
            communication_tx:   com_thread_tx,
            communication_rx:   com_thread_rx,
            process_id:         None,
            thread_type:        ThreadType::OutputCaptureThread,
            thread_active:      false,
            name:               name_output
        }, audio_tx, com_rx, com_tx)
    }

    pub fn input_hardware_template(block_size: usize, name: &str) -> ( 
        Self, 
        mpsc::Receiver<Vec<u8>>,
        mpsc::Receiver<CommandInterThread>, 
        mpsc::Sender<CommandInterThread>) 
    {
        let (audio_tx, audio_rx) = mpsc::channel();
        let (com_tx, com_thread_rx) = mpsc::channel();
        let (com_thread_tx, com_rx) = mpsc::channel();
        let name_input = String::from(name);
        return (Self {
            audio_block_size:   block_size,
            audio_channel_rx:   None,
            audio_channel_tx:   Some(audio_tx),
            communication_tx:   com_thread_tx,
            communication_rx:   com_thread_rx,
            process_id:         None,
            thread_type:        ThreadType::InputCaptrureThread,
            thread_active:      false,
            name:               name_input
        }, audio_rx, com_rx, com_tx)
    }

    /// Create Audio struct with all fields filled before calling this function
    pub fn audio_thread(&mut self) {
        match self.thread_type {
            ThreadType::ApplicationCaptureThread => {
                match self.audio_channel_tx {
                    Some(_) => (),
                    None    => {
                        error!("Audio tx channel is not created but needed for: {:?}", self.thread_type);
                        self.communication_tx
                            .send(CommandInterThread::ErrorStartingKillingSelf)
                            .expect("");
                        return;
                    },
                }
                match self.audio_channel_rx {
                    Some(_) => {
                        warning!("Audio rx channel not needed for: {:?}. Closing channel...", self.thread_type);
                        self.audio_channel_rx = None;
                    },
                    None => (),
                }
                match self.process_id {
                    Some(id) => {
                        if id == 0 {
                            error!("Process id can not be 0 for: {:?}", self.thread_type);
                            self.communication_tx
                                .send(CommandInterThread::ErrorStartingKillingSelf)
                                .expect("");
                            return;
                        }
                    },
                    None => {
                        error!("Need a process id for a {:?}", self.thread_type);
                        self.communication_tx
                            .send(CommandInterThread::ErrorStartingKillingSelf)
                            .expect("");
                        return;
                    }
                }
                self.main_application(); 
            },
            ThreadType::InputCaptrureThread => {
                match self.audio_channel_tx {
                    Some(_) => (),
                    None    => {
                        error!("Audio tx channel needed for: {:?}. Killing self", self.thread_type);
                        self.communication_tx
                            .send(CommandInterThread::ErrorStartingKillingSelf)
                            .expect("");
                        return;
                    },
                }
                match self.audio_channel_rx {
                    Some(_) => {
                        warning!("Audio rx channel not needed for: {:?}. Closing channel...", self.thread_type);
                        self.audio_channel_rx = None;
                    },
                    None    => (),
                }
                match self.process_id {
                    Some(_) => {
                        warning!("Process id not needed for: {:?}. Procces id will be removed", self.thread_type);
                        self.process_id = None;
                    },
                    None    => (),
                }
                self.main_input()
            },
            ThreadType::OutputCaptureThread => {
                match self.audio_channel_tx {
                    Some(_) => {
                        warning!("Audio tx channel not needed for: {:?}. Closing channel...", self.thread_type);
                        self.audio_channel_tx = None;
                    },
                    None => ()
                }
                match self.audio_channel_rx {
                    Some(_) => (),
                    None    => {
                        error!("Audio rx channel needed for: {:?}. Killing self", self.thread_type);
                        self.communication_tx
                            .send(CommandInterThread::ErrorStartingKillingSelf)
                            .expect("");
                        return;
                    },
                }
                match self.process_id {
                    Some(_) => {
                        warning!("Process id not needed for: {:?}. Procces id will be removed", self.thread_type);
                        self.process_id = None;
                    },
                    None    => (),
                }
                self.main_output()
            },
        }
    }
    
    pub fn get_stream_type(&self) -> ThreadType {
        self.thread_type.clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn main_application(&mut self) {
        self.thread_active = true;
        info!("Application thread started: {}", self.name);
        loop {
            
        }
        self.thread_active = false;
    }

    fn main_output(&mut self) {
        self.thread_active = true;
        info!("Output thread started: {}", self.name);
        loop {

        }
        self.thread_active = false;
    }

    fn main_input(&mut self) {
        self.thread_active = true;
        info!("Input thread started: {}", self.name);
        loop {

        }
        self.thread_active = false;
    }

    fn suspend_thread(&mut self) -> bool {
        loop {
            let msg = self.communication_rx.recv().unwrap();
            match msg {
                CommandInterThread::GetStatus => {
                    self.communication_tx
                    .send(CommandInterThread::StatusSuspended)
                    .unwrap();
                    continue;
                },
                CommandInterThread::ErrorStartingKillingSelf => return true,
                CommandInterThread::ResumeAudioStream => return false,
                _ => (),
            } 
        }
    }
}
