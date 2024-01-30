use std::io::BufReader;
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

pub struct Nscriptsound{
    threadssenders: HashMap<String,mpsc::Sender<String>>,
    durations: HashMap<String,Duration>,
    //timers: HashMap<String,Duration>,
    instants: HashMap<String,Instant>,
    allsoundids: String,
    soundint:usize,
    currentvolume: f32
}

impl Nscriptsound{
    pub fn new() -> Nscriptsound {
        Nscriptsound {
            threadssenders: HashMap::new(),
            durations: HashMap::new(),
            //timers: HashMap::new(),
            instants: HashMap::new(),
            allsoundids:  String::new(),
            soundint: 0,
            currentvolume: 1.0

        }
    }

    pub fn playfile(&mut self,filepath: &str)->String{
        let path = Path::new(&filepath);
        let duration = match mp3_duration::from_path(&path){
            Ok(res) =>{
res
            },
            Err(err) => Duration::from_secs(0)
        };
        if duration == Duration::from_secs(0)
 {
            println!("Error getting the duration of :{}",&filepath);
            return "error".to_owned();
        }
        let thisid = self.play(filepath.to_string());
        self.durations.insert(thisid.clone().to_string(),duration);

        self.allsoundids = "".to_owned() + &self.allsoundids + &thisid + "|";
        //println!("adding:{}",thisid);
        //println!("arr:{}",self.allsoundids);
        let start_time = Instant::now();
        self.instants.insert(thisid.clone(),start_time);
        //let elapsed_time = start_time.elapsed();
        thisid
    }
    pub fn getduration(&mut self,musicid:&str)->String{
let mut ret = String::new();
        if let Some(duration) = self.durations.get(musicid) {
        ret = format!("{:?}",duration)
        };
if ret != ""{
            return ret;
        }
String::from("error")

    }
    pub fn runtimers(&mut self){
        if self.allsoundids == "" {
            return
        }
        let mut tostop = String::new();
        let array: Vec<&str>= self.allsoundids.split("|").collect();


        for musicid in  array{

            if let Some(lastelapsed_time) = self.instants.get(musicid) {
                if let Some(duration) = self.durations.get(musicid) {

                    if lastelapsed_time.elapsed() >= duration.to_owned() {

                        tostop = tostop + musicid +"|";
                    }
                }
            }

        }
        if tostop != ""{
            let len = tostop.len();
            if Nscriptsound::fromright(&tostop,1) == "|"{
                tostop = String::from(&tostop[0..len-1]);
            }
            let array: Vec<&str>= tostop.split("|").collect();
            for stopid in array{
                //println!("removed soundthread {}",&stopid);
                self.stop(&stopid);

            }
        }

    }
     fn fromright(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }
    fn create_soundid(&mut self)->String{
        self.soundint = self.soundint + 1;
            if self.soundint > 999{
                self.soundint = 0;
            }

        let newid = "[".to_owned()+ &self.soundint.to_string() + "]";

        newid
    }

    fn create_thread(&mut self) -> (mpsc::Sender<String>,mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        // Do something with tx if needed
        (tx, rx)
    }
     fn play(&mut self,soundfile:String) -> String{
        // internally used, spawns the audio thread and returns its channels
        let soundfilecl = soundfile.clone();
        let (tx, rx)  = self.create_thread();
        thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            //println!("file:{}",&soundfilecl);
            let file = std::fs::File::open(soundfilecl.clone()).unwrap();
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(decoder);

            loop {

                if sink.empty() {
                    break;
                }
                let received_message = rx.recv().unwrap();
                match received_message.as_str(){
                    "mute" =>{
                        sink.set_volume(0.0);
                    }
                    "unmute" =>{
                        sink.set_volume(1.0);
                    }
                    "stop" => {

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


        tx.send("st".to_string()).unwrap();
       let thisid = self.create_soundid();

        self.threadssenders.insert(thisid.clone().to_string(),tx);
        thisid
    }
    pub fn setvolume(&mut self,id:&str,vol: f32){
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("volume>".to_string()+&vol.to_string());
                self.currentvolume = vol;
            }
    }
    pub fn mute(&mut self,id: &str){
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("volume>0.0".to_string());
             }
    }
    pub fn unmute(&mut self,id: &str){
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("volume>".to_string()+&self.currentvolume.to_string());
            }
    }


    pub fn stop(&mut self,id:&str){
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("stop".to_string());
                let torep = "".to_owned() + &id +"|";
                self.allsoundids = self.allsoundids.replace(&torep,"");
            }
    }
}

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



