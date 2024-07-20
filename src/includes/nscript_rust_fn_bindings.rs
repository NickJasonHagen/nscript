
use crate::*;

pub fn nscript_callfn(
    func: &str,
    param1: &str,
    param2: &str,
    param3: &str,
    param4: &str,
    param5: &str,
    param6: &str,
    param7: &str,
    param8: &str,
    param9: &str,
    vmap: &mut Varmap,
) -> String {
    // translate nscript calls towards the runtime functions and return vallues as string
    // all calls must be from String and back to String new adds be required to do so aswell
    // !! this is where you add your functions to the nscript syntaxis, if you have this included in your rust project
    // --------------------------------------------------------------------------------------

    // this is where you can override the built in functions with your custom functions and extent the
    // current functions with your own, just make a switch on the func and the others are given
    // arguments from the .nc calls
    // let customfunctions = Nscriptfunctions::nscript_custom_functions(&func,&param1,&param2,&param3,&param4,&param5,&param6,&param7,&param8,&param9,vmap);
    // if customfunctions != "" {
    //     return customfunctions
    // }
    let  custom_behavior: NscriptCustomFunctions = vmap.fnextentions;
    let customret = custom_behavior(vmap);
    if customret != ".."{
        return customret.to_owned();
    }

    match func {
        // "scope" => {
        //     return "RET=>".to_owned() + &nscript_unpackscope(param2,param1,vmap)
        // }
        "nearest_even" => {
            return Ncmath::nearest_even_number(&param1);
        }
        "square_root" => {
            return Ncmath::square_root_str(&param1);
        }
        "percentage" => {
            return Ncmath::percentage(&param1,&param2);
        }

        "objtofile" => {
           return nscript_objecttofile(&param1,&param2, vmap);
        }
        "filetoobj" => {
           return nscript_filetoobject(&param1,&param2, vmap);
        }
        "soundvolume" =>{
            match param2.parse::<f32>(){
                Ok(res) => {
                    vmap.sound.setvolume(param1,res);

                    vmap.setvar("___error".to_string(),"false");
                    return "ok".to_string();

                }
                Err(_) => {
                    vmap.setvar("___error".to_string(),"true");
                    return "coulnd parse the given volume argument as a f32, make sure the number be like 0.0".to_string();
                }
            };
        }
        "soundplay" =>{
            vmap.sound.play(param1);
            return "".to_string();
        }
        "soundstop" =>{
            vmap.sound.stop(param1);
            return "".to_string();
        }
        "soundclose" =>{
            vmap.sound.stop(param1);
            return "".to_string();
        }
        "soundmute" =>{
            vmap.sound.mute(param1);
            return "".to_string();
        }
        "soundunmute" =>{
            vmap.sound.unmute(param1);
            return "".to_string();
        }

        "join" =>{
            return param1.replace(param2,NC_ARRAY_DELIM);
        }
        "mp3duration" => {

            return vmap.sound.getduration(&param1);
        }
        "playsoundfile" => {

             return vmap.sound.playfile(&param1);
        }
        "stopsound" =>{
            vmap.sound.stop(&param1);
            return "".to_owned();
        }
        "tcplistener" => {
            return vmap.ntcp.listener( param1,param2);
        }
        "tcpaccept" => {
            return vmap.ntcp.accept(param1);
        }
        "tcpconnect" => {
            return vmap.ntcp.connect(param1,param2);
        }
        "tcpsend" => {
            return vmap.ntcp.send(param1,param2);
        }
        "tcpreceive" => {
            return vmap.ntcp.receive(param1);
        }
        "tcpdisconnect" => {
            return vmap.ntcp.disconnect(param1);
        }
        "terminalenableraw" => {
            Nterminal::enableraw();
            return String::new();
        }
        "terminaldisableraw" => {
            Nterminal::disableraw();
            return String::new();
        }

        "download" =>{
            let mut port: u16 = 80;
            if let Ok(porta) = param2.to_owned().parse::<u16>() {
                println!("Parsed port number: {}", port);
                port = porta;
                // Use the port number (port) here in your code
            } else {
                port = 80;
                println!("Failed to parse port number");
                // Handle the case when parsing fails
            }
            let toret = match get_http_file_content(param1, port,param3,param4) {
                Ok(_) => format!("File download  @{}", param4),
                Err(err) => format!("Error: {}", err),
            };
            return toret.to_string();
        }
        "httppost" => {
            let port: u16;
            if let Ok(porta) = param2.to_owned().parse::<u16>() {
                //println!("Parsed port number: {}", port);
                // Use the port number (port) here in your code
                port = porta;
            } else {
                port = 80;
                //println!("Failed to parse port number");
                // Handle the case when parsing fails
            }
            return httppost(param1,port,param3,param4);
        }
        "httpget" => {
            let port: u16;
            if let Ok(porta) = param2.to_owned().parse::<u16>() {
                //println!("Parsed port number: {}", port);
                // Use the port number (port) here in your code
                port = porta;
            } else {
                port = 80;
                //println!("Failed to parse port number");
                // Handle the case when parsing fails
            }

            match get_http_content(param1, port,param3) {
                Ok(data) => {
                    if let Ok(string) = String::from_utf8(data) {
                        return string;
                    } else {
                        return "httpget: Failed to convert to string".to_string();
                    }
                },
                Err(err) => return err.to_string(),
            }
        }
        "ncwebserver" => {
            match ncwebserver(vmap){
                Ok(_) => return String::new() ,
                Err(_) => return String::new(),
            }

        }
        // "rawget" => {
        //     match raw_http_get(param1,param2) {
        //         Ok(response) => return response ,
        //         Err(_) => return String::new(),
        //     }
        // }
        // "rawgetfile" => {
        //     match raw_http_get_file(param1,param2,param3) {
        //         Ok(response) => return response ,
        //         Err(_) => return "rawgetfile fileerror".to_string(),
        //     }
        // }
        "restrictionmode" => {
            nscript_setrestrictionmode(&param1,vmap);
            return "".to_owned() + &param1;
        }
        "debugmode" => {
            nscript_setdebugmode(&param1,vmap);
            return "".to_owned() + &param1;
        }
        "math" | "calc" => {
            let res = nscript_runmath(&split(&param1," "),0, vmap);
            return res;
        }
        "run" => {
             call_program(&param1,&param2,&param3,&param4,&param5,&param6,&param7,&param8,&param9);
            return "ran".to_owned();
        }
        "runwait" => {
            return call_programwait(&param1,&param2,&param3,&param4,&param5,&param6,&param7,&param8,&param9);
        }
        // "sqlgetrow" => {
        //     return perform_sql_query_get_row(&param1,&param2,param3);
        // }
        // "decrypt" => {
        //     return encrypt_string(&param1,&param2);
        // }
        //
        // "encrypt" => {
        //     return decrypt_string(&param1,&param2);
        // }
        "trim" => {
            return param1.trim().to_string();
        }
        "minutes_in_ms" => {
            return Ntimer::minutes_in_ms(&param1);
        }
        "hours_in_ms" => {
            return Ntimer::hours_in_ms(&param1);
        }
        "days_in_ms" => {
            return Ntimer::days_in_ms(&param1);
        }
        "weeks_in_ms" => {
            return Ntimer::weeks_in_ms(&param1);
        }
        "months_in_ms" => {
            return Ntimer::months_in_ms(&param1);
        }
        "years_in_ms" => {
            return Ntimer::years_in_ms(&param1);
        }

        "stringtobase64" => {
            return string_to_base64(&param1);
        }
        "filetobase64" => {
            return file_to_base64(&param1);
        }
        "base64tofile" => {
            return base64_to_file(&param1,&param2);
        }

        "base64tostring" => {
            return base64_to_string(&param1);
        }
        "unzip" => {
            return unzip_file(&param1,&param2);
        }
        "zip" => {
            return zip_directory(&param1,&param2);
        }
        "memusage" => {
            return memoryusage();
        }
        "memstats" | "memorystatus" => {
            return memorystatus();
        }
        "objtojson" => {
            return objtojson(&param1,vmap);
        }
        "objfromjson" => {
            objfromjson(&param1,param2,vmap);
            return param1.trim().to_owned();
        }
        "dirdelete" => {
            return directory_delete(&param1);
        }
        "dirmove" => {
            return directory_move(&param1,&param2);
        }
        "arrayreverse" => {
            return arrayreverse(&param1);
        }
        "arrayfirstout" => {
            return arrayfirstout(&param1);
        }
        "arraylastout" => {
            return arraylastout(&param1);
        }

        "arraypush" => {
            return arraypush(&param1,&param2);
        }
        "string_to_hexplus" =>{
            return Nstring::tohexplus(param1);
        }
        "string_from_hexplus" =>{
            return Nstring::fromhexplus(param1);
        }

        "threadsend" => {
             match vmap.threadssenders.get(param1){
                Some(sender) => {
                match sender.send(param2.to_string()){
                    Ok(_)=>{
                            //println!("main send succes!");
                            match vmap.threadsreceiver.get(param1){
                                Some(receiver) =>{
                                   let msg: String = match receiver.try_recv(){
                                        Ok(m) =>m,
                                        Err(_) =>"".to_owned()
                                    };
                                    match msg.as_str(){
                                        _ =>{
                                            if msg.as_str() != ""{
                                                //println!("main sent{} received:{}",param2,msg);
                                                return msg;

                                            }

                                        }
                                    }
                                },


                                None => {
                                    println!("no thread [{}] receiver channel found!",&param1);
                                }
                            }

                        },
                    Err(_)=>{

                            //println!("main[{}] send error! msg({})",&param1,&param2);
                            return "error".to_string();
                        }
                };
                    return "ok".to_owned();
                }
                None => {
                    println!("no threads found");
                    return "".to_owned();
                }
            };
        }
        "arraypushroll" => {
            return arraypushroll(&param1,&param2);
        }
        "terminalinput" => {
            return terminal_get_user_input(&param1,&param2);
        }
        "updatedterminal" =>{
            Nterminal::updatedterminal(param1);
            return "".to_owned();
        }
        "terminalkey" =>{

            return Nterminal::terminalkey();
        }
        "codelevel" => {
            return vmap.codelevel.to_string();
        }
        // "discordmsg" => {
        //     send_message_to_discord_api(&param1, &param2);
        //     return String::new();
        // }
        "filecopy" | "fcopy" => {
            return filecopy(&param1,&param2);
        }
        "filedelete" | "fdelete" => {
            return filedelete(&param1);
        }

        "filemove" | "fmove"=> {
            return filemove(&param1,&param2);
        }
        "round" => {
            return round_number(&param1,&param2);
        }
        "random" => {
            return random_number_between(&param1,&param2,&param3);
        }
        "identifierarray" => {
            return identifierarray(&param1,&param2,&param3);
        }
        "dircreate" => {
            return create_directory(&param1);
        }
        "filesizebytes" => {
            return filesizebytes(&param1);
        }

        "filesize" => {
            return filesize(&param1);
        }
        // "curl" => {
        //     return curl(&param1);
        //
        // }
        "iscode" => {
            let ret = vmap.getcode(&param1);
            cwrite(&ret,"red");
            return String::from(&ret);

        }
        "inpool" => {
            return inpool(&param1,&param2);
        }
        "pooladd" => {
            return pooladd(&param1,&param2);
        }
        "poolremove" => {
            return poolremove(&param1,&param2);
        }
        "arraysort" => {
            return arraysort(&param1);
        }
        "stackpush" => {
            vmap.stackpush(param1, param2);
            return String::new();
        }
        "sleep" => {
            if let Ok(duration) = param1.parse::<u64>() {
                std::thread::sleep(std::time::Duration::from_millis(duration));
            } else {
                // Invalid argument, handle the error
                return String::from("Invalid argument for sleep function");
            }
            return String::from("") // Return an empty string as the result
        }
        "hextostring" => {
            //vmap.stackpush(param1, param2);
            return hex_to_string(param1);
        }
        "stringtohex" => {
            //vmap.stackpush(param1, param2);
            return string_to_hex(param1);
        }
        "stackpop" => {
            return vmap.stackpop(param1);
        }
        "delobj"  | "objdel" => {
            // execute deconstruct function (if is has it)
            let isdeconfn = "".to_owned() + &param1 + ".deconstruct()"; // should only execute if it exists.. else continue
            nscript_func(&isdeconfn, vmap);
            vmap.delobj(param1);
            return String::new();
        }
        "objparents" => {
            return vmap.objparents(param1);
        }
        "objchildren" | "getobjchildren"=> {
            return vmap.objchildren(param1);
        }
        "setobjprop" => {
            vmap.setprop(param1, param2, param3);
            return String::new();
        }
        "getobjprop" => {
            let get = vmap.getprop(param1, param2);
            return get;
        }
        "setobj" => {
            vmap.setobj(param1, param2);
            let isconfn = "".to_owned() + &param1 + ".construct()"; // should only execute if it exists.. else continue
            nscript_func(&isconfn, vmap);
            return String::new();
        }
        "inobj" => {
            return Nstring::replace(&vmap.inobj(param1),"|",NC_ARRAY_DELIM);
        }

        "delobjprop" => {
            vmap.delprop(param1, param2);
            return String::new();
        }
        "stringtoeval" => {
            return Nstring::stringtoeval(param1);
        }
        "isfunction" => {
            let testc = vmap.getcode(param1);
            //println!("isfunction:{}",testc);
            return nscript_parsesheet(&testc, vmap);
        }
        "exec" => {
            nscript_execute_script(
                param1, param2, param3, param4, param5, param6, param7, param8, param9, "", vmap,
            );
            return "ok".to_owned();
        }
        "sheet" => {
            return nscript_parsesheet(&Nfile::read(param1), vmap);
        }
        "code" => {
            return nscript_parsesheet(&param1, vmap);
        }

        "cin" => {
            return param1.to_string();
        }
        "browseropen" => {
            browseropen(param1);
            return String::new();
        }
        "chain" => {
            // internally used when chaining multiple functions of a object
            return nscript_runchains(&split(param1," "), vmap);
        }
        "cwrite" | "print" => {
            cwrite(param1, param2);
            return param1.to_owned();
        }
        "cwriteraw" | "printraw" => {
            cwriteraw(param1, param2);
            io::stdout().flush().unwrap();
            return param1.to_owned();
        }
        "printpos" =>{
            let x:u16 = match param3.parse::<usize>(){
                Ok(res) =>{
                    res.try_into().unwrap_or(1)
                }
                Err(_) =>{
                    1
                }
            };
            let y:u16 = match param4.parse::<usize>(){
                Ok(res) =>{
                    res.try_into().unwrap_or(1)
                }
                Err(_) =>{
                    1
                }
            };
            Nterminal::print(param1,param2,x,y);
            return "".to_owned();
        }
        "terminalflush" => {
            Nterminal::flush();
            return "".to_owned();
        }
        "len" => {
            return split(param1,NC_ARRAY_DELIM).len().to_string();
        }
        "timerinit" => {
            return Ntimer::init().to_string();
        }
        "timerdiff" => {
            let ret = match param1.parse::<i64>(){

                Ok(r) => {
                    Ntimer::diff(r)
                },
                Err(_) => {
                    //println!("timererror:{}",e);
                    //println!("timer var:{}",param1);
                    0

                }
            };
            return ret.to_string();

        }
        "fread" | "fileread" => {
            return Nfile::read(param1);
        }
        "fwrite" | "filewrite" => {
            let var = Nfile::write(param1, param2);
            return var;
        }
        "splitselect" => {

            return splitselect(&param1,&param2,parse_string_to_usize(&param3))
        }
        "file_read_utf8" =>{
            return read_file_utf8(&param1).to_owned();
        }
        "fexists" | "fileexists"=> {
            if Nfile::checkexists(&param1) == true {
                return String::from("true");
            } else {
                return String::from("false");
            }
        }
        "listdir" | "dirtolist" | "dirlist" => {
            if param2 == "" {
                return Nfile::dirtolist(param1, false);
            } else {
                return Nfile::dirtolist(param1, true);
            }
        }

        "split" => {
            return Nstring::split(param1, param2);
        }
        "instring" => {
            if Nstring::instring(param1, param2) == true {
                return String::from("true");
            } else {
                return String::from("false");
            }
        }
        "replace" => {
            //println!("replace a{} b{} c{}",&param1,&param2,&param3);
            //            let ret = param1.to_owned().replace(param2,param3);
            let ret = Nstring::replace(param1,param2,param3);
            //println!("replaced: [{}]",&ret);

            return ret;            //return Nstring::replace(param1, param2, param3);
        }
        "trimleft" => {
            return Nstring::trimleft(param1, param2.parse::<usize>().unwrap());
        }
        "trimright" => {
            return Nstring::trimright(param1, param2.parse::<usize>().unwrap());
        }

        "fromleft" => {
            return Nstring::fromleft(param1, param2.parse::<usize>().unwrap());
        }
        "fromright" => {
            return Nstring::fromright(param1, param2.parse::<usize>().unwrap());
        }
        "save" => {
            Njh::write(param1, param2, param3);
            return String::new();
        }
        "load" => {
            return Njh::read(param1, param2);
        }
        "setvar" => {
            vmap.setvar(param1.to_string(), param2);
            return String::new();
        }
        "getvar" => {
            return vmap.getvar(param1);
        }

        "exit" => {
            return String::from("exit");
        }
        "arrayfilter" => {
            return arrayfilter(param1,param2);
        }
        "arraysearch" => {
            return arraysearch(param1,param2);
        }
        "arrayshuffle" => {
            return arrayshuffle(param1);
        }
        "" => {
            return String::new();
            //required?!
        }
        "decode_html_url" => {
            return decode_html_url(&param1).to_string();
        }
        "html_encode" => {
            return html_encode(&param1);
        }
        "arraystartwith" =>{
            return arraystartwith(&param1, &param2);
        }
        "arrayendwith" =>{
            return arrayendwith(&param1, &param2);
        }

        "int64" => {
            let res = match param1.parse::<i64>(){
                Ok(_) => "true",
                Err(_) => "false",
            };
            return res.to_owned();
        }
        "int32" => {
            let res = match param1.parse::<i32>(){
                Ok(_) => "true",
                Err(_) => "false",
            };
            return res.to_owned();
        }

        "f64" => {
            let res = match param1.parse::<f64>(){
                Ok(_) => "true",
                Err(_) => "false",
            };
            return res.to_owned();
        }
        "add" => {
            return nscript_quickmath("+", &param1, &param2, &param3, &param4, &param5, &param6, &param7, &param8, &param9, vmap);
        }
        "subtract" => {
            return nscript_quickmath("-", &param1, &param2, &param3, &param4, &param5, &param6, &param7, &param8, &param9, vmap);
        }
        "devide" => {
            return nscript_quickmath("/", &param1, &param2, &param3, &param4, &param5, &param6, &param7, &param8, &param9, vmap);
        }
        "multiply" => {
            return nscript_quickmath("*", &param1, &param2, &param3, &param4, &param5, &param6, &param7, &param8, &param9, vmap);
        }
        "f32" => {
            let res = match param1.parse::<f32>(){
                Ok(_) => "true",
                Err(_) => "false",
            };
            return res.to_owned();
        }
        "tolowercase" =>{
            return param1.to_string().to_lowercase();
        }
        "touppercase" =>{
            return param1.to_string().to_lowercase();
        }

        "keytest" => {

            keytest("w");
            return  "".to_owned();
        }
        "stringbetween" => return Nstring::stringbetween(param1, param2, param3),
        "combine" | "cat"=> {
            let nothing = param1.to_owned()
            + param2
            + param3
            + param4
            + param5
            + param6
            + param7
            + param8
            + param9;
            return nothing;
        }
        _ => {
            let error = "".to_owned() + "A non declared function call is done:" + &func;
            nscript_interpreterdebug(&error,vmap.debugmode,vmap.strictness);
            return String::new();
            // debug broken/non existing call
        }
    };


}


