use anyhow::Result;
use rodio::{
    source::{Repeat, TakeDuration},
    Decoder, OutputStream, OutputStreamHandle, Sink, Source,
};
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::vk::{self, VK};

pub struct Player {
    pub wavs: HashMap<String, TakeDuration<Repeat<Decoder<BufReader<File>>>>>,
    pub sinks: Vec<Sink>,
    _streams: Vec<(OutputStream, OutputStreamHandle)>,
}
impl Player {
    pub fn new() -> Self {
        let mut v_sink = Vec::new();
        let mut v_stream = Vec::new();
        for _ in 0..5 {
            let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle);
            v_stream.push((stream, stream_handle));
            v_sink.push(sink.unwrap());
        }
        Self {
            wavs: HashMap::new(),
            sinks: { v_sink },
            _streams: v_stream,
        }
    }
    pub fn load_dir_by_json(&mut self) -> Result<()> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open("./resource/kv_wav_map.json")?;
        let map: HashMap<String, String> = serde_json::from_reader(file)?;
        for i in map {
            self.load_file(i.0, i.1)?;
        }
        Ok(())
    }

    pub fn load_file(&mut self, wav: String, path: String) -> Result<()> {
        let file = std::fs::OpenOptions::new().read(true).open(path)?;
        let buf = BufReader::new(file);
        let src = rodio::Decoder::new(buf)?;
        let one_time = src.total_duration().unwrap();
        let many_src = src.repeat_infinite().take_duration(one_time);

        self.wavs.insert(wav, many_src);
        Ok(())
    }

    pub fn play(&mut self, vk: &VK) {
        if let vk::Action::Release = vk.action {
            return;
        }
        let mut sink = None;
        for i in self.sinks.iter_mut() {
            if i.empty() {
                sink = Some(i);
                break;
            }
        }
        if sink.is_none() {
            return;
        }
        let sink = sink.unwrap();
        let target = format!("default{}", vk.vk_code % 5);

        if let Some(value) = &vk.value {
            if let Some(wav) = self.wavs.get(value) {
                sink.append(wav.clone());
            } else {
                sink.append(self.wavs.get(&target).unwrap().clone());
            }
        } else {
            sink.append(self.wavs.get(&target).unwrap().clone());
        }
        //    println!("{}", self.sinks.len());
    }
}
