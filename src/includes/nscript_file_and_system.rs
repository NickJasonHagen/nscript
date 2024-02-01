use crate::*;
use sysinfo::{System };
use psutil::process::Process;
use std::process;
use std::process::Command;
use std::io::{self, Write,Read};
use crossterm::{
    execute,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal, ExecutableCommand,
};
use std::{thread, time};

use termion::{
    event::{Event, Key, MouseButton, MouseEvent},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    cursor,
    clear,
};pub fn keytest(thiskey: &str) {
    // Hide the cursor and enable raw mode for handling input
    let _ = terminal::enable_raw_mode();

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
                // Detect specific key presses
                let thiskey = "w";
                match code {
                    // KeyCode::Char(thiskey) if modifiers == KeyModifiers::CONTROL => {
                    //     print!("keyend:{}",thiskey);
                    //     break; // Exit when Ctrl + q is pressed
                    // }
                    KeyCode::Char(thiskey) => {
                        print!("keyend:{}",thiskey);
                        break; // Exit when Ctrl + q is pressed
                    }
                    _ => {
                        // Handle other key presses here
                        let mystr = format!("Pressed: {:?}", code);
                        //let mystr = code.to_string();
                        println!("format{}",mystr);
                    }
                }
            }
        }
    }

    // Disable raw mode and show the cursor when done
    let _ = terminal::disable_raw_mode();
}
pub fn browseropen(site: &str) {
    // Execute the xdg-open command with the URL
    let result = Command::new("xdg-open")
        .arg(site)
        .output();

    // Check if the command executed successfully
    match result {
        Ok(_) => println!("Opened the URL in the default browser."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
pub struct Nterminal{

}

impl Nterminal{
    pub fn enableraw(){
        let _ = terminal::enable_raw_mode();
    }
    pub fn disableraw(){
        let _ = terminal::disable_raw_mode();
    }
    pub fn updatedterminal(printframe:&str){
        // Set up the terminal
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = io::BufWriter::new(stdout);
        //let stdin = io::stdin();

        print!("{}{}", clear::All, cursor::Hide);
        stdout.flush().unwrap();
        let mut i =  1;

        for line in split(printframe,"\n"){
            let mut beginline = 1;
            for subline in split(line,"|||"){
                let checkcolor = split(subline,"&printcolor=");
                if checkcolor.len() > 1{
                    Nterminal::print(checkcolor[0],checkcolor[1],beginline,i);
                    let lenght: u16 =  split(checkcolor[0],"").len() as u16;
                    beginline = beginline + lenght ;


                }
                else{
                    print!(
                    "{}{}",
                    cursor::Goto(1,i),
                    line
                );
                }


            }
            i = i +1;
        }
        stdout.flush().unwrap();
        //thread::sleep(Duration::from_secs(1));
        // Restore the terminal state
        print!("{}{}", cursor::Show, clear::All);
    }
    pub fn terminalkey()->String{
        // Listen for keyboard input in the main thread
        let mut ret = String::new();

         //let soundthread = thread::spawn(move || {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = io::BufWriter::new(stdout);
        let stdin = io::stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('e') =>{
                    ret = "e".to_owned();
                    break
                },
                Key::Char('f') =>{
                    ret = "f".to_owned();
                    break
                },
                Key::Char('g') =>{
                    ret = "g".to_owned();
                    break
                },
                Key::Char('h') =>{
                    ret = "h".to_owned();
                    break
                },
                Key::Char('j') =>{
                    ret = "j".to_owned();
                    break
                },
                Key::Char('k') =>{
                    ret = "k".to_owned();
                    break
                },
                Key::Char('l') =>{
                    ret = "l".to_owned();
                    break
                },
                Key::Char('m') =>{
                    ret = "m".to_owned();
                    break
                },
                Key::Char('n') =>{
                    ret = "n".to_owned();
                    break
                },
                Key::Char('o') =>{
                    ret = "o".to_owned();
                    break
                },
                Key::Char('p') =>{
                    ret = "p".to_owned();
                    break
                },
                Key::Char('q') =>{
                    ret = "q".to_owned();
                    break
                },
                Key::Char('r') =>{
                    ret = "r".to_owned();
                    break
                },
                Key::Char('s') =>{
                    ret = "s".to_owned();
                    break
                },
                Key::Char('t') =>{
                    ret = "t".to_owned();
                    break
                },
                Key::Char('v') =>{
                    ret = "v".to_owned();
                    break
                },
                Key::Char('w') =>{
                    ret = "w".to_owned();
                    break
                },
                Key::Char('x') =>{
                    ret = "x".to_owned();
                    break
                },
                Key::Char('y') =>{
                    ret = "y".to_owned();
                    break
                },
                Key::Char('z') =>{
                    ret = "z".to_owned();
                    break
                },
                Key::Char('\n') =>{
                    ret = "enter".to_owned();
                    break
                },
                Key::Char(' ') =>{
                    ret = "space".to_owned();
                    break
                },

                Key::Up =>{
                    ret =  "up".to_owned();
                    break
                },
                Key::Down =>{
                    ret =  "down".to_owned();
                    break
                },
                Key::Left =>{
                    ret =  "left".to_owned();
                    break
                },
                Key::Right =>{
                    ret =  "right".to_owned();
                    break
                },

                Key::BackTab=>{
                    ret =  "tab".to_owned();
                    break
                },
                Key::Backspace =>{
                    ret =  "backspace".to_owned();
                    break
                },
                Key::Esc =>{
                    ret =  "esc".to_owned();
                    break
                },
                _ => {

                }
            }
            //stdout.flush().unwrap();
        }
        stdout.flush().unwrap();
//});
        return ret;

    }
    fn print(m:&str,color:&str,x:u16,i:u16){
    match color {
        "bright blue" | "bb" => {

            print!(
            "{}{}",
            cursor::Goto(1,i),
            m.bright_blue()
        );
        }
        "bright green" | "bg"=> {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_green());
        }
        "bright cyan" | "bc" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_cyan());
        }
        "bright red" | "br" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_red());
        }
        "bright magenta" | "bm" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_magenta());
        }
        "bright yellow" | "by" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_yellow());
        }
        "bright purple" | "bp" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.bright_purple());
        }
        "purple" | "p" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.purple());
        }
        "cyan" | "c" =>{
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.cyan());
        }
        "yellow" | "y" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.yellow());
        }
        "red" | "r" => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.red());
        }
        "green" | "g" => {
                       print!(
            "{}{}",
            cursor::Goto(x,i), m.green());
        }
        "blue" | "b" =>{
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.blue());
        }
        "magenta" | "m" =>{
                        print!(
            "{}{}",
            cursor::Goto(x,i), m.magenta());
        }

        _ => {
                        print!(
            "{}{}",
            cursor::Goto(x,i), m);

        }
    };

    }
}
pub struct Nfile {
    // nscript filesystem
}

impl Nfile {
    pub fn dirtolist(readpath: &str, fullpathnames: bool) -> String {
        let mut output = String::new();

        let paths = match fs::read_dir(readpath) {
            Ok(paths) => paths,
            Err(error) => {
                println!("<error>: Cannot read directory: {}", error);
                return String::new();
            }
        };

        for path in paths {
            match path {
                Ok(entry) => {
                    let unwraped = entry.path().display().to_string();
                    if !unwraped.is_empty() {
                        output.push_str(&unwraped);
                        output.push_str(NC_ARRAY_DELIM);
                    }
                }
                Err(error) => {
                    println!("<error>: Cannot access directory entry: {}", error);
                    return String::new();
                }
            }
        }

        if !fullpathnames {
            output = output.replace(readpath, "");
        }

        if Nstring::fromright(&output, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
            return Nstring::trimright(&output, NC_ARRAY_DELIM.len());
        }

        output
    }
    pub fn checkexists(fp: &str) -> bool {
        return std::path::Path::new(fp).exists();
    }
    pub fn write(path: &str, data: &str) -> String {
        if std::path::Path::new(&path).exists(){
            let  _error = match fs::remove_file(path) {
                Ok(_) => format!("File deleted successfully"),
                Err(err) =>{
                    return format!("Error writing a file ( cant delete,before write): {}", err);
                } ,
            };


        }
        let mut f = match File::create(path) {
            Ok(file) => file,
            Err(err) => return err.to_string(),
        };

        if let Err(err) = f.write_all(data.as_bytes()) {
            return err.to_string();
        }

        if let Err(err) = f.sync_all() {
            return err.to_string();
        }

        String::new()
    }
    pub fn read(floc: &str) -> String {
    let mut file = match File::open(floc) {
        Ok(file) => file,
        Err(_) => return String::new(), // Return empty string on error
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return String::new(); // Return empty string on error
    }

    kill_bill(&contents)
    }
    pub fn readraw(floc: &str) -> String {
        let contents = fs::read_to_string(floc);
        match &contents {
            Err(_e) => String::new(),
            Ok(t) => String::from(&read_to_string(&t)),
        }
    }
}


pub fn create_directory(dir_path: &str) -> String {
    match fs::create_dir(dir_path) {
        Ok(_) => format!("Directory '{}' created successfully", dir_path),
        Err(err) => format!("Error creating directory: {:?}", err),
    }
}

// Move a file from the source path to the destination path
pub fn filemove(source: &str, destination: &str) -> String {
    if source == "" || destination == ""{
        return "Aruments cannot be empty!!".to_string()
    }
    match fs::rename(source, destination) {
        Ok(_) => format!("File moved successfully"),
        Err(err) => format!("Error moving file: {}", err),
    }
}

// Copy a file from the source path to the destination path
pub fn filecopy(source: &str, destination: &str) -> String {
    match fs::copy(source, destination) {
        Ok(_) => format!("File copied successfully"),
        Err(err) => format!("Error copying file: {}", err),
    }
}

// Delete a file at the specified path
pub fn filedelete(file: &str) -> String {
    match fs::remove_file(file) {
        Ok(_) => format!("File deleted successfully"),
        Err(err) => format!("Error deleting file: {}", err),
    }
}

// Delete a directory and all its contents
pub fn directory_delete(directory: &str) -> String {
    match fs::remove_dir_all(directory) {
        Ok(_) => format!("Directory deleted successfully"),
        Err(err) => format!("Error deleting directory: {}", err),
    }
}

// Move a directory from the source path to the destination path
pub fn directory_move(source: &str, destination: &str) -> String {
    match fs::rename(source, destination) {
        Ok(_) => format!("Directory moved successfully"),
        Err(err) => format!("Error moving directory: {}", err),
    }
}

pub fn memorystatus()->String{
        let mut system = System::new_all();

    // Refresh the system to get the latest information
    system.refresh_all();

    // Get system memory information
    let total_memory = system.total_memory();
    let free_memory = system.free_memory();
    let used_memory = total_memory - free_memory;

    // Print the memory information
    let toreturn = "Total: ".to_owned() + &total_memory.to_string() + " KB\nUsed:"
    + &used_memory.to_string() + "KB\nFree:" + &free_memory.to_string() + "KB";
    return toreturn;
}

pub fn memoryusage() -> String{
    if let Some(memory_usage) = get_process_memory_usage() {
        //println!("Process Memory Usage: {} bytes", memory_usage);
        return memory_usage.to_string();
    } else {
        //println!("Failed to get process memory usage.");
        return "error".to_string();
    }
}
pub fn get_own_pid() -> u32 {
    // Get the current process ID using std::process::id()
    process::id()
}

pub fn get_process_memory_usage() -> Option<u64> {
    // Get the current process ID
    let pid = get_own_pid();

    // Try to get the process by its ID
    if let Ok(process) = Process::new(pid) {
        // Get the memory information for the process
        if let Ok(mem_info) = process.memory_info() {
            // Return the RSS (Resident Set Size) in bytes
            return Some(mem_info.rss());
        }
    }

    // Return None if there was an error or the process is not found
    None
}

pub struct Nc_os{

}
 impl Nc_os{

    pub fn envvarget(var:&str)->String{
        let ret: String;
        if let Ok(value) = env::var(var) {
            ret = value;
        } else {
            ret = "~/nscript".to_owned();
        }
        ret
    }
        pub fn envvars()->String{
            let mut ret = String::new();
            // Iterate over all environment variables
            for (key, value) in env::vars() {
                ret = ret +"\n" + &format!("{}: {}", key, value);
            }
            ret
        }
}
// fn convert_array<T: std::fmt::Display>(array: &[T]) -> String {
//     let elements: Vec<String> = array.iter().map(|element| element.to_string()).collect();
//     format!("{{{}}}", elements.join(", "))
// }
//
// fn dll_open(path: &str) -> Result<*mut c_void, Box<dyn Error>> {
//     let path_cstring = CString::new(path)?;
//     let handle = unsafe { libc::dlopen(path_cstring.as_ptr(), libc::RTLD_NOW) };
//     if handle.is_null() {
//         Err(format!("Failed to open DLL: {:?}", path).into())
//     } else {
//         Ok(handle)
//     }
// }
//
// fn dll_close(handle: *mut c_void) {
//     unsafe {
//         libc::dlclose(handle);
//     }
// }
//
// fn dllcall(handle: *mut c_void, func_name: &str, args: &[(&str, &str)]) -> Result<String, Box<dyn Error>> {
//     let func_name_cstring = CString::new(func_name)?;
//     let func_ptr = unsafe { libc::dlsym(handle, func_name_cstring.as_ptr()) };
//     if func_ptr.is_null() {
//         return Err(format!("Failed to find function: {:?}", func_name).into());
//     }
//
//     let func: extern "C" fn(c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int) -> *mut c_int =
//         unsafe { std::mem::transmute(func_ptr) };
//
//     let mut converted_args: [c_int; 10] = [0; 10];
//     for (i, &(arg_type, arg_value)) in args.iter().enumerate() {
//         let converted_arg = match arg_type {
//             "int" => convert_arg(arg_value)?,
//             _ => return Err(format!("Unsupported argument type: {:?}", arg_type).into()),
//         };
//         converted_args[i] = converted_arg;
//     }
//
//     let result_ptr = func(converted_args[0], converted_args[1], converted_args[2], converted_args[3], converted_args[4],
//                           converted_args[5], converted_args[6], converted_args[7], converted_args[8], converted_args[9]);
//
//     let result = unsafe { *result_ptr };
//     if result == 0 {
//         return Err("Function call failed".into());
//     }
//
//     let result_array: &[c_int] = unsafe { std::slice::from_raw_parts(result_ptr, result as usize + 1) };
//
//     if result_array.len() > 1 {
//         let array_string = convert_array(&result_array[1..]);
//         Ok(array_string)
//     } else {
//         Ok(result.to_string())
//     }
// }
