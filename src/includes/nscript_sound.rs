use std::io::{BufReader, BufRead};
use std::time::Duration;
//use rodio::Source;
use std::sync::mpsc;
use std::collections::HashMap;
//use media::Media;
//extern crate id3;
use std::thread;

use mp3_duration;
use std::path::Path;
//use cpal::traits::{DeviceTrait, StreamTrait};
use std::time::Instant;

use crate::split;

use crate::*;



pub struct Ncplayer{
    threadssenders: HashMap<String,mpsc::Sender<String>>,
    durations: HashMap<String,Duration>,
    instants: HashMap<String,Instant>,
    allsoundids: String,
    soundint:usize,
    currentvolume: f32,
}

impl Ncplayer{
    pub fn new() -> Ncplayer {
        Ncplayer {
            threadssenders: HashMap::new(),
            durations: HashMap::new(),
            instants: HashMap::new(),
            allsoundids:  String::new(),
            soundint: 0,
            currentvolume: 1.0,


        }
    }

    pub fn playfile(&mut self,filepath: &str)->String{
        // begins the whole things,
        let path = Path::new(&filepath);
        // get and set duration
        let duration = mp3_duration::from_path(&path).unwrap_or(Duration::from_secs(0));
        let thisid = self.spawnaudiothread(filepath.to_string());
        self.durations.insert(thisid.clone().to_string(),duration);
        //Add to activesoundbuffer and set timer
        self.allsoundids = "".to_owned() + &self.allsoundids + &thisid + "|";
        let start_time = Instant::now();
        self.instants.insert(thisid.clone(),start_time);
        thisid
    }
     pub fn getduration(&mut self,id:&str)->String{
        if let Some(ret) = self.durations.get(id){
            return format!("{:?}",ret.to_owned());
        }
        "0".to_string()
    }
    //  fn duration(&mut self,id:&str)->Duration{
    //     if let Some(ret) = self.durations.get(id){
    //         return ret.to_owned();
    //     }
    //     Duration::from_secs(0)
    // }
    // pub fn elapsed(&mut self,id: &str)->Duration{
    //     if let Some(elapsed_time) = self.instants.get(id) {
    //         if let Some(duration) = self.durations.get(id) {
    //             Duration::from(elapsed_time.elapsed());
    //         }
    //     }
    //    Duration::from_secs(0)
    //
    // }
    pub fn runtimers(&mut self){
        // use this inside your apps loops, this will handle the sounds by killing the threads when
        if self.allsoundids == "" {
            return
        }
        //stop() String buffer
        let mut tostop = String::new();
        // get array of all active sounds and check their timers
        let array: Vec<&str>= self.allsoundids.split("|").collect();
          for musicid in  array{
            if let Some(lastelapsed_time) = self.instants.get(musicid) {
                if let Some(duration) = self.durations.get(musicid) {
                    if lastelapsed_time.elapsed() >= duration.to_owned() {
                        // Stringbuffer with the music ids which will be stopped.
                        tostop = tostop + musicid +"|";
                    }
                }
            }

        }
        // handle stop() buffer.
        if tostop != ""{
            let len = tostop.len();
            if Ncplayer::fromright(&tostop,1) == "|"{
                tostop = String::from(&tostop[0..len-1]);
            }
            let array: Vec<&str>= tostop.split("|").collect();
            for stopid in array{
                self.soundclose(&stopid);

            }
        }

    }

     fn fromright(s: &str, f: usize) -> String {
        // strips a number of chrs from a string
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }

    fn create_soundid(&mut self)->String{
        //1k sounds max, gonna get crazy resets itself so play 1000 at the same time max
        self.soundint = self.soundint + 1;
            if self.soundint > 999{
                self.soundint = 0;
            }
        // create a unique id to search and replace
        // since we counting we add a prefix and suffix cause we do replacements on strings.
        let newid = "[".to_owned()+ &self.soundint.to_string() + "]";
        newid
    }

    fn create_thread(&mut self) -> (mpsc::Sender<String>,mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        (tx, rx)
    }

    fn spawnaudiothread(&mut self,soundfile:String) -> String{
        //interally used to spawn threads to play the audio
        let soundfilecl = soundfile.clone();
        let (tx, rx)  = self.create_thread();
        thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let file = BufReader::new(std::fs::File::open(soundfilecl.clone()).unwrap());

            let decoder = rodio::Decoder::new(file).unwrap();
            sink.append(decoder);

            loop {

                // if sink.empty() {
                //     break;
                // }
                let received_message = rx.recv().unwrap();
                match received_message.as_str(){
                    "mute" =>{
                        sink.set_volume(0.0);
                    }
                    "play" =>{
                        //sink.stop();
                        sink.play();

                        //sink.append(decoder);
                    }
                    "unmute" =>{
                        sink.set_volume(1.0);
                    }
                    "stop" => {

                        sink.stop();
                    }
                    "exit" => {

                        break;
                    }
                    _ =>{
                        if received_message.as_str().contains("volume>"){
                            match split(received_message.as_str(),">")[1].parse::<f32>(){
                                Ok(r) =>{
                                    sink.set_volume(r);

                                }
                                Err(_) =>{

                                }
                            };
                        }
                    }

                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        let thisid = self.create_soundid();
        self.threadssenders.insert(thisid.clone().to_string(),tx);
        thisid
    }

    pub fn setvolume(&mut self,id:&str,vol: f32){
        match self.threadssenders.get(id) {
            Some(sender)  => {
                match sender.send("volume>".to_string()+&vol.to_string()){
                    Ok(_)=>{},
                    Err(_)=>{}
                };
                self.currentvolume = vol;

            }
            None =>{

            }
        }
    }

    pub fn mute(&mut self,id: &str){

        match self.threadssenders.get(id) {
            Some(sender)  => {
                match sender.send("volume>0.0".to_string()){
                    Ok(_)=>{},
                    Err(_)=>{}
                };
                self.currentvolume = 0.0;
            }
            None =>{

            }
        }
    }
    pub fn unmute(&mut self,id: &str){
        match self.threadssenders.get(id) {
            Some(sender) =>{
                match sender.send("volume>".to_string()+&self.currentvolume.to_string()){
                    Ok(_)=>{},
                    Err(_)=>{}
                };

            }
            None =>{}
        }
    }
    pub fn play(&mut self,id: &str){
        match self.threadssenders.get(id) {
            Some(sender) =>{
                match sender.send("play".to_string()){
                    Ok(_)=>{},
                    Err(_)=>{}
                };

            }
            None =>{}
        }
    }
    pub fn stop(&mut self,id:&str){
        // used to stop active sounds,
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("stop".to_string());
        }
    }
    pub fn soundclose(&mut self,id:&str){
        // used to stop active sounds,
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("stop".to_string());
            let torep = "".to_owned() + &id +"|";
            self.allsoundids = self.allsoundids.replace(&torep,"");
        }
    }
}


//
// fn main() {
//     let mut play = nscriptsound::new();
//     play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//      play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//    play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//
//     loop {
//         //thread::sleep(Duration::from_secs(3));
//         //play.stop("stop");
//         //println!("oi");
//         //break;
//     play.runtimers();
//     }
// }
//


