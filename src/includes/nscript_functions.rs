use base64::{encode, decode};
//use rusqlite::{Connection, Result};
//time
use chrono::{Datelike, Timelike};
use crate::NC_ARRAY_DELIM;
use crate::*;





// Function to convert a string to base64
pub fn string_to_base64(string_in: &str) -> String {
    encode(string_in)
}

// Function to convert base64 to a string
pub fn base64_to_string(string_in: &str) -> String {
    match decode(string_in) {
        Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
        Err(_) => String::new(),
    }
}
pub fn file_to_base64(file_path: &str) -> String {
    // Attempt to open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            // Return an empty string if there's an error opening the file
            return String::new();
        }
    };

    // Read the file contents into a buffer
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer) {
        // Return an empty string if there's an error reading the file
        return String::new();
    }

    // Encode the buffer to base64
    let base64_string = encode(&buffer);

    base64_string
}
pub fn base64_to_file(base64_string: &str, file_path: &str) -> String {
    // Decode the base64 string
    let decoded_bytes = match decode(base64_string) {
        Ok(bytes) => bytes,
        Err(_) => {
            return format!("Error: Failed to decode base64");
        }
    };

    // Write the decoded bytes to the file
    match File::create(file_path) {
        Ok(mut file) => {
            if let Err(_) = file.write_all(&decoded_bytes) {
                return format!("Error: Failed to write to file");
            }
        }
        Err(_) => {
            return format!("Error: Failed to create file");
        }
    };

    "ok".to_string()
}

pub fn read_file_utf8(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return String::new();
    }

    let (decoded, _, _) = UTF_8.decode(&contents);
    decoded.into_owned()
}

pub fn parse_string_to_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(parsed_number) => parsed_number,
        Err(_) => 0,
    }
}

pub fn splitselect(arrayvar: &str,delim: &str,entree: usize) -> String{
    let this = split(&arrayvar,&delim);
    if entree > this.len()-1 {
        String::new()
    }
    else{
        return this[entree].to_string()
    }
}

pub fn terminal_get_user_input(message: &str, default: &str) -> String {
    print!("D:{} \n{}: ", default, message);
    io::stdout().flush().unwrap(); // Flushes the output to ensure the message is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove trailing newline character
    input = input.trim_end().to_string();

    if input.is_empty() {
        default.to_string()
    } else {
        input
    }
}

pub fn round_number(number: &str, decimals: &str) -> String {
    match (number.parse::<f64>(), decimals.parse::<usize>()) {
        (Ok(parsed_number), Ok(parsed_decimals)) => {
            let rounded = parsed_number.round();
            let formatted = format!("{:.*}", parsed_decimals, rounded);
            formatted
        }
        _ => String::new(),
    }
}
pub fn filesizebytes(file: &str) -> String {
    // returns the full byte size of a file!
    let path = Path::new(file);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return String::new(),
    };

    let realsize = metadata.len();

    realsize.to_string()
}

pub fn filesize(file: &str) -> String {
    // returns a fancy calculated string of the size rounded GB/MB/KB
    let path = Path::new(file);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return String::new(),
    };

    let realsize = metadata.len();
    if realsize >= 1_000_000_000 {
        return format!("{:.2} GB", realsize as f64 / 1_000_000_000.0);
    }
    if realsize >= 1_000_000 {
        return format!("{:.2} MB", realsize as f64 / 1_000_000.0);
    }
    if realsize >= 1_000 {
        return format!("{:.2} KB", realsize as f64 / 1_000.0);
    }

    format!("{} B", realsize)
}

pub fn random_number_between(min: &str, max: &str, decimals: &str) -> String {
    let min_num = match min.parse::<f64>() {
        Ok(parsed_num) => parsed_num,
        Err(_) => return String::new(),
    };

    let max_num = match max.parse::<f64>() {
        Ok(parsed_num) => parsed_num,
        Err(_) => return String::new(),
    };

    if min_num > max_num {
        return String::new();
    }

    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(min_num..=max_num);

    if decimals.is_empty() {
        return random_num.to_string();
    }

    let rounded_num = match decimals.parse::<usize>() {
        Ok(num_decimals) => format!("{:.*}", num_decimals, random_num),
        Err(_) => return String::new(),
    };

    rounded_num
}



pub fn call_program(command: &str,arg1: &str,arg2: &str,arg3: &str,arg4: &str,arg5: &str,arg6: &str,arg7: &str,arg8: &str) -> String {
    // let mut parts = command.split_whitespace();
    // let program = parts.next().expect("No program provided");
    // let args: Vec<_> = parts.collect();
    //
    let output = Command::new(arg1).arg(arg2).arg(arg3).arg(arg4).arg(arg5).arg(arg6).arg(arg7).arg(arg8).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Program executed successfully.\nStdout: {}\nStderr: {}", stdout, stderr)
            } else {
                format!("Program execution failed with exit code: {:?}", output.status.code())
            }
        }
        Err(err) => {
            format!("Failed to execute program: {}", err)
        }
    }
}
pub fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    // returns a str array vector
    let r: Vec<&str> = s.split(p).collect();
    return r;
}
pub fn splittostringvec(s:&str,p:&str) -> Vec<String>{
    // returns a String array vector
     let vec_string: Vec<String> = split(s,p).iter().map(|&s| s.to_string()).collect();
    return vec_string;
}

pub fn cwrite(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "bright blue" | "bb" => {
            println!("{}", m.bright_blue());
        }
        "bright green" | "bg"=> {
            println!("{}", m.bright_green());
        }
        "bright cyan" | "bc" => {
            println!("{}", m.bright_cyan());
        }
        "bright red" | "br" => {
            println!("{}", m.bright_red());
        }
        "bright magenta" | "bm" => {
            println!("{}", m.bright_magenta());
        }
        "bright yellow" | "by" => {
            println!("{}", m.bright_yellow());
        }
        "bright purple" | "bp" => {
            println!("{}", m.bright_purple());
        }
        "purple" | "p" => {
            println!("{}", m.purple());
        }
        "cyan" | "c" =>{
            println!("{}", m.cyan());
        }
        "yellow" | "y" => {
            println!("{}", m.yellow());
        }
        "red" | "r" => {
            println!("{}", m.red());
        }
        "green" | "g" => {
            println!("{}", m.green());
        }
        "blue" | "b" =>{
            println!("{}", m.blue());
        }
        "magenta" | "m" =>{
            println!("{}", m.magenta());
        }

        _ => {
            println!("{}", m);

        }
    };
}
pub fn cwriteraw(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "bright blue" | "bb" => {
            print!("{}", m.bright_blue());
        }
        "bright green" | "bg"=> {
            print!("{}", m.bright_green());
        }
        "bright cyan" | "bc" => {
            print!("{}", m.bright_cyan());
        }
        "bright red" | "br" => {
            print!("{}", m.bright_red());
        }
        "bright magenta" | "bm" => {
            print!("{}", m.bright_magenta());
        }
        "bright yellow" | "by" => {
            print!("{}", m.bright_yellow());
        }
        "bright purple" | "bp" => {
            print!("{}", m.bright_purple());
        }
        "purple" | "p" => {
            print!("{}", m.purple());
        }
        "cyan" | "c" =>{
            print!("{}", m.cyan());
        }
        "yellow" | "y" => {
            print!("{}", m.yellow());
        }
        "red" | "r" => {
            print!("{}", m.red());
        }
        "green" | "g" => {
            print!("{}", m.green());
        }
        "blue" | "b" =>{
            print!("{}", m.blue());
        }
        "magenta" | "m" =>{
            print!("{}", m.magenta());
        }

        _ => {
            print!("{}", m);

        }
    };
}

pub struct Ncmath{

}
impl Ncmath{
    pub fn nearest_even_number(num: &str) ->String {
        let num2 = match num.parse::<f32>(){
            Ok(res) =>{
                res
            }
            Err(_) =>{
                0.0
            }
        };
        let rounded_num = num2.round() as i32;
        let ret = rounded_num / 2 * 2;
        ret.to_string()
    }

    pub fn square_root_str(input: &str) -> String{
        // Parse the input string to a number
        let num = match input.parse::<f64>() {
            Ok(num) => num,
            Err(_) => return "NaN".to_owned(),
        };

        // Calculate the square root
        let square_root = num.sqrt();

        // Convert the square root to a string
        let result = square_root.to_string();

        result
    }

    pub fn percentage(max_str: &str, tocalc_str: &str) -> String {
        // Parse the input strings to numbers
        let max = match max_str.parse::<f64>(){
            Ok(max) => max,
            Err(_) => return String::from("NaN"),
        };
        let tocalc = match tocalc_str.parse::<f64>() {
            Ok(tocalc) => tocalc,
            Err(_) => return String::from("NaN"),
        };

        // Calculate the percentage
        let percent = (tocalc / max) * 100.0;

        // Convert the percentage to a string
        percent.to_string()
    }
}
pub fn nscript_quickmath(method:&str,a:&str,b:&str,c:&str,d:&str,e:&str,f:&str,g:&str,h:&str,i:&str,vmap:&mut varmap) -> String{
    let mut res = nscript_math(a,"+",b,vmap);
    let method = "+";
    if c != "" {res = nscript_math(&res, &method, &c, vmap);}
    if d != "" {res = nscript_math(&res, &method, &d, vmap);}
    if e != "" {res = nscript_math(&res, &method, &e, vmap);}
    if f != "" {res = nscript_math(&res, &method, &f, vmap);}
    if g != "" {res = nscript_math(&res, &method, &g, vmap);}
    if h != "" {res = nscript_math(&res, &method, &h, vmap);}
    if i != "" {res = nscript_math(&res, &method, &i, vmap);}
    res
}

// pub fn perform_sql_query_get_row(query: &str, getrow: &str,database: &str) -> String {
//     let conn = Connection::open(database).unwrap();
//     let mut stmt = conn.prepare(query).unwrap();
//     let rows = stmt.query_map([], |row| {
//         // Process each row of the result set
//         // Modify this closure to extract the desired values from the row
//         // For example, if the row contains a text column called 'name':
//         let name: String = row.get(getrow)?;
//         Ok(name)
//     }).unwrap();
//
//     let mut result = String::new();
//
//     for row in rows {
//         let name: String = row.unwrap().to_string();
//         result.push_str(&name);
//         result.push('\n');
//     }
//
//     result
// }
