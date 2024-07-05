use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use std::sync::mpsc;

use wasapi::*;
use crate::basic_audio_driver_rust::{ 
    audio_stream::AudioStream, 
    combine_streams::CombineStreams, 
    modulation_stream::AudioModulation, 
    CommandInterThread,
    ThreadType
};
use crate::log_system::*;

struct SoundTile {
    audio_stream:       AudioStream,
    communication_tx:   mpsc::Sender<CommandInterThread>,
    communication_rx:   mpsc::Receiver<CommandInterThread>,
    audio_tx:           Option<mpsc::Sender<Vec<u8>>>,
    audio_rx:           Option<mpsc::Receiver<Vec<u8>>>,
}

pub struct SoundBoard {
    audio_streams_list: HashMap<String, SoundTile>,

}

impl SoundBoard {
    pub fn init() -> Self {
        Self {
            audio_streams_list: HashMap::new(),
        }
    }

    pub fn update_hardware_devices(&mut self) {
        let borrow_list = &DeviceCollection::new(&Direction::Render); 
        let device_collection = match borrow_list {
            Ok(data)    => data,
            Err(err)    => {
                error!("Can not update output device list. Error: {}", err);
                return;
            }
        };

        let mut vec = vec![];
        for dev in device_collection {
            let device      = dev.unwrap();
            let name        = &device.get_friendlyname().unwrap();
            if self.audio_streams_list.contains_key(name) == false {
                let (stream, audio_tx, com_rx, com_tx) = 
                AudioStream::output_hardware_template(4096, name);
                let sound_tile = SoundTile {
                    audio_stream:       stream,
                    audio_rx:           None,
                    audio_tx:           Some(audio_tx),
                    communication_tx:   com_tx,
                    communication_rx:   com_rx
                };
                self.audio_streams_list.insert(name.to_string(), sound_tile);
            }
            vec.push(name.clone());
        }

        let borrow_list = &DeviceCollection::new(&Direction::Capture);
        let device_collection = match borrow_list {
            Ok(data)    => data,
            Err(err)    => {
                error!("Can not update input device list. Error: {}", err);
                return;
            }
        };

        for dev in device_collection {
            let device      = dev.unwrap();
            let name        = &device.get_friendlyname().unwrap();
            if self.audio_streams_list.contains_key(name) == false {
                let (stream, audio_rx, com_rx, com_tx) = 
                AudioStream::input_hardware_template(4096, name);
                let sound_tile = SoundTile {
                    audio_stream:       stream,
                    audio_rx:           Some(audio_rx),
                    audio_tx:           None,
                    communication_tx:   com_tx,
                    communication_rx:   com_rx
                };
                self.audio_streams_list.insert(name.to_string(), sound_tile);
            }
            vec.push(name.clone());
        }
        self.filter_audio_list(&vec);
    }

    fn filter_audio_list(&mut self, key_list: &Vec<String>) {
        let mut vec = vec![];
        for (key, _) in self.audio_streams_list.iter() {
            vec.push(key.clone());
        }
        for key in vec {
            if !key_list.contains(&key) {
                let stream = match self.audio_streams_list.get(&key) {
                    Some(stream) => stream,
                    None => continue,
                };
                match stream.audio_stream.get_stream_type() {
                    ThreadType::ApplicationCaptureThread => (),
                    _ => {
                        verbose!("Removed: {}", key);
                        let _ = self.audio_streams_list.remove(&key);
                    }
                }
            }
        }
    }

    pub fn get_hardware_output_list(&self) -> Vec<&str> {
        let mut vec = Vec::new();
        for (_key, tile) in self.audio_streams_list.iter() {
            let stream = &tile.audio_stream;
            let name = stream.get_name();
            match stream.get_stream_type() {
                ThreadType::OutputCaptureThread => vec.push(name),
                _ => ()
            }
        }
        vec
    }
}

impl Default for SoundBoard {
    fn default() -> Self {
        let mut sound_board = Self::init();
        let _ = sound_board.update_hardware_devices();
        Self {
            audio_streams_list: sound_board.audio_streams_list,
        }
    }
}
