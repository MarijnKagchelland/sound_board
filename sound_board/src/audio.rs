use std::string::String;
use std::ops::Add;
use std::vec::Vec;
use cpal::{Device, DevicesError};
use cpal::traits::{DeviceTrait, HostTrait};
use egui::ahash::HashSet;
use crate::{verbose, info, warning, error, debug};
use crate::log_system::LogLevels;
use crate::log_system::{Local, OpenOptions, BufWriter, Colorize, Write};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum AudioBroadTypes {
    HardwareInput,
    VirtualInput,
    Modulation,
    VirtualOutput,
    HardwareOutput,
}

#[derive(Clone)]
pub struct AudioStream {
    audio_broad_type: AudioBroadTypes,
    device: Option<Device>,
}

impl AudioStream {
    fn new(audio_broad_type: AudioBroadTypes, device: Option<Device>) -> Self {
        Self {
            device,
            audio_broad_type
        }
    }
}

pub struct SoundBoard {
    hardware_audio_output: Vec<AudioStream>,
    hardware_audio_input:  Vec<AudioStream>,
    virtual_audio_output:  Vec<AudioStream>,
    virtual_audio_input:   Vec<AudioStream>,
}

impl SoundBoard {
    pub fn init() -> Self {
        Self {
            virtual_audio_input: Vec::new(),
            virtual_audio_output: Vec::new(),
            hardware_audio_input: Vec::new(),
            hardware_audio_output: Vec::new()
        }
    }
    pub fn update_hardware_devices(&mut self) -> Result<(), DevicesError> {
        let mut host = cpal::default_host();
        let input_devices_list = match host.input_devices() {
            Err(val) => {
                error!("Input device error: {:?}", val);
                return Err(val);
            }
            Ok(input_devices) => {
                input_devices
            },
        };
        let output_devices_list = match host.output_devices() {
            Err(val) => {
                error!("output device error: {:?}", val);
                return Err(val);
            }
            Ok(input_devices) => {
                input_devices
            },
        };
        let mut input_devices = Vec::new();
        for device in input_devices_list {
            input_devices.push(device);
        }
        let mut output_devices = Vec::new();
        for device in output_devices_list {
            output_devices.push(device);
        }
        let current_input_device: Vec<_> = input_devices.iter().map(|d| d.clone()).collect();
        let current_output_device: Vec<_> = output_devices.iter().map(|d| d.clone()).collect();

        let mut filtered_in_audio_streams = Vec::new();
        for audio_stream in self.hardware_audio_input.clone() {
            for device in &current_input_device {
                if String::from(device.name().unwrap()) == String::from(&audio_stream.device.as_ref().unwrap().name().unwrap()) {
                    filtered_in_audio_streams.push(AudioStream::new(HardwareInput, Some(device.clone())));
                    break;
                }
            }
        }
        let mut filtered_out_audio_streams: Vec<_> = self.hardware_audio_output.clone().into_iter()
            .filter(|audio_stream| {
                for device in &current_output_device {
                    if String::from(device.name().unwrap()) == String::from(&audio_stream.device.clone().unwrap().name().unwrap()) {
                        return true;
                    }
                }
                false})
                .collect();

        let existing_input_devices: Vec<_> = filtered_in_audio_streams.iter()
            .map(|audio_stream| audio_stream.device.as_ref().unwrap().clone())
            .collect();
        let existing_output_devices: Vec<_> = filtered_out_audio_streams.iter()
            .map(|audio_stream| audio_stream.device.as_ref().unwrap().clone())
            .collect();
        let mut new_input_devices: Vec<_> = current_input_device.iter()
            .filter(|d| {
                for device in &existing_input_devices {
                    if String::from(device.name().unwrap()) != d.name().unwrap() {
                        return true;
                    }
                }
            false}).collect();
        let mut new_output_devices: Vec<_>= current_output_device.iter()
            .filter(|d| {
                for device in &existing_output_devices {
                    if String::from(device.name().unwrap()) != d.name().unwrap() {
                        return true;
                    }
                }
                false}).collect();

        self.hardware_audio_input.clear();
        self.hardware_audio_output.clear();
        self.hardware_audio_input = filtered_in_audio_streams;
        self.hardware_audio_output = filtered_out_audio_streams;

        for mut device in new_input_devices {
            self.hardware_audio_input.push(AudioStream::new(HardwareInput, Some(device.clone())));
        }
        for device in new_output_devices {
            self.hardware_audio_output.push(AudioStream::new(HardwareOutput, Some(device.clone())));
        }
        let mut string = String::from("New hardware input devices list:");
        for audio_stream in self.hardware_audio_input.clone() {
            string = string.add(format!("\n\r\t{}", audio_stream.device.unwrap().name().unwrap()).as_str());
        }
        verbose!("{}", string);
        let mut string = String::from("New hardware output devices list:");
        for audio_stream in self.hardware_audio_output.clone() {
            string = string.add(&format!("\n\r\t{}", audio_stream.device.unwrap().name().unwrap()).as_str());
        }
        verbose!("{}", string);
        Ok(())
    }
}