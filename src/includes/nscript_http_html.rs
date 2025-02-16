use crate::*;
//use reqwest;
//use reqwest::blocking::get;
//use std::time::Duration;
use std::time::Instant;

// pub fn curl(url: &str) -> String {
//     match get(url) {
//         Ok(mut response) => {
//             let mut content = String::new();
//             if let Ok(_) = response.read_to_string(&mut content) {
//                 return content;
//             }
//         }
//         Err(err) => eprintln!("Error: {:?}", err),
//     }
//     String::new()
// }
//
// pub fn raw_http_get(url: &str,fname: &str) -> Result<String, Box<dyn std::error::Error>> {
//
//     //format for TCP socket
//     let spliturl =  split(&url,":");
//     let host = &spliturl[0];
//     let mut port = "80";
//     if spliturl.len() > 1 {
//         port = spliturl[1];
//     }
//     let addr = format!("{}:{}", &host, &port)
//         .to_socket_addrs()?
//         .next()
//         .ok_or("Unable to resolve the hostname")?;
//
//     // Connect to the server
//
//     let mut stream = TcpStream::connect_timeout(&addr,Duration::from_secs(4))?;
//     match stream.set_read_timeout(Some(Duration::new(0, 420000000))){
//
//         Ok(_) => {},
//         Err(_) => println!("[nctcphttp] Error setting the stream read timeout"),
//     }
//
//     // let err = result.unwrap_err();
//     // assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
//     match stream.set_write_timeout(Some(Duration::new(0, 420000000))){
//
//         Ok(_) => {},
//         Err(_) => println!("[nctcphttp] Error setting the stream write timeout"),
//     }
//     // create the GET header
//     let msg = "GET /".to_owned() + &fname + " HTTP/1.1
// Host:" + &format!("{}:{}", &host, &port) +"
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
// Accept-Encoding: gzip, deflate
// Accept-Language: en-US,en;q=0.9
// Connection: keep-alive
// ";
//     // send the Get request to the server, the socket will send meta back
//     // close and the next stream be the dataloop.
//     let byte_slice: &[u8] = msg.as_bytes();
//     stream.write_all(&byte_slice)?;
//
//     // Receive response from the server
//     let mut buffer = [0; 1024];
//     let bytes_read = stream.read(&mut buffer)?;
//     let rawres = String::from_utf8_lossy(&buffer[0..buffer.len()]);
//     // get the size of what we going to receive.
//     let bytesize = Nstring::stringbetween(&rawres,"Content-Length: ","\n");
//     let buffer_size: usize = bytesize.parse().unwrap_or(1024);
//     // create a new bytebuffer with the size of the contents
//     let mut buffer = vec![0; buffer_size];
//     let mut receivedbytes = 0; // used as counter++
//     // this string be filled with the data read as bytes.
//     let mut receivedstring = String::new();
//     let mut timerdc = Ntimer::init();
//     loop {
//         match stream.read(&mut buffer){
//             Ok(e) => {
//                 let bytes_read = e;
//                 receivedbytes += bytes_read;
//                 receivedstring = receivedstring + &String::from_utf8_lossy(&buffer[0..bytes_read]);
//                 timerdc = Ntimer::init();
//
//             },
//             Err(_) => {
//                 println!("rawget Streamread error");
//                 break;
//             },
//         };
//
//
//         if Ntimer::diff(timerdc) >= 1999 {
//             println!("rawget timedout 2seconds., exit loop proceed code.");
//             return Ok("timedout".to_string());
//         }
//         if bytes_read == 0 {
//             // this is a socket close / end of packet / error.
//             break;
//         }
//     }
//
//     //Nfile::write("./testget.txt",&receivedstring);
//     Ok(receivedstring.to_string())
// }
// pub fn raw_http_get_file(url: &str,fname: &str,saveas: &str) -> Result<String, Box<dyn std::error::Error>> {
//
//     //format for TCP socket
//     let spliturl =  split(&url,":");
//     let host = &spliturl[0];
//     let mut port = "80";
//     if spliturl.len() > 1 {
//         port = spliturl[1];
//     }
//     let addr = format!("{}:{}", &host, &port)
//         .to_socket_addrs()?
//         .next()
//         .ok_or("Unable to resolve the hostname")?;
//
//     // Connect to the server
//     let mut stream = TcpStream::connect_timeout(&addr,Duration::from_secs(4))?;
//     match stream.set_read_timeout(Some(Duration::new(0, 420000000))){
//
//         Ok(_) => {},
//         Err(_) => println!("[nctcphttp] Error setting the stream read timeout"),
//     }
//
//     // let err = result.unwrap_err();
//     // assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
//     match stream.set_write_timeout(Some(Duration::new(0, 420000000))){
//
//         Ok(_) => {},
//         Err(_) => println!("[nctcphttp] Error setting the stream write timeout"),
//     }
//     // create the GET header
//     let msg = "GET /".to_owned() + &fname + " HTTP/1.1
// Host:" + &format!("{}:{}", &host, &port) +"
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
// Accept-Encoding: gzip, deflate
// Accept-Language: en-US,en;q=0.9
// Connection: keep-alive
// ";
//     // send the Get request to the server, the socket will send meta back
//     // close and the next stream be the dataloop.
//     let byte_slice: &[u8] = msg.as_bytes();
//     stream.write_all(&byte_slice)?;
//
//     // Receive response from the server
//     let mut buffer = [0; 1024];
//     let bytes_read = stream.read(&mut buffer)?;
//     let rawres = String::from_utf8_lossy(&buffer[0..buffer.len()]);
//     // get the size of what we going to receive.
//     let bytesize = Nstring::stringbetween(&rawres,"Content-Length: ","\n");
//     let buffer_size: usize = bytesize.parse().unwrap_or(1024);
//     // create a new bytebuffer with the size of the contents
//     let mut buffer = vec![0; buffer_size];
//     let mut receivedbytes = 0; // used as counter++
//     // this string be filled with the data read as bytes.
//     let mut receivedstring = String::new();
//     let mut timerdc = Ntimer::init();
//     loop {
//         match stream.read(&mut buffer){
//             Ok(e) => {
//                 let bytes_read = e;
//                 receivedbytes += bytes_read;
//                 receivedstring = receivedstring + &String::from_utf8_lossy(&buffer[0..bytes_read]);
//                 timerdc = Ntimer::init();
//
//             },
//             Err(_) => {
//                 println!("rawget Streamread error");
//                 break;
//             },
//         };
//
//         if bytes_read == 0 {
//             // this is a socket close / end of packet / error.
//             break;
//         }
//         if Ntimer::diff(timerdc) >= 1999 {
//             println!("rawget timedout 2seconds., exit loop proceed code.");
//             break;
//         }
//
//     }
//     Nfile::write(saveas,&receivedstring);
//     Ok("FiledownloadComplete".to_string())
// }
pub fn httppost(ipaddr: &str, port: u16, url: &str, postdata: &str) -> String{
    // Connect to the server
    //
    let mut stream = match TcpStream::connect((ipaddr, port)) {
        Ok(stream) => stream,
        Err(err) => return format!("Failed to connect: {}", err),
    };
                // match stream.set_read_timeout(Some(Duration::new(0, 1000000000))){
                //
                //     Ok(_) => {},
                //     Err(_) => println!("[nctcphttp] Error setting the stream read timeout"),
                // }
                //
                // // let err = result.unwrap_err();
                // // assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
                // match stream.set_write_timeout(Some(Duration::new(0, 1000000000))){
                //
                //     Ok(_) => {},
                //     Err(_) => println!("[nctcphttp] Error setting the stream write timeout"),
                // }


    let resultstring: String;
    // Prepare the HTTP POST request
    let request = format!("POST {} HTTP/1.1\r\n\
        Host: {}:{}\r\n\
        Content-Type: application/x-www-form-urlencoded\r\n\
        Content-Length: {} Cache\r\n\
        Connection: close\r\n\
        \r\n\
        {}\r\n", url, ipaddr, port, postdata.len(), postdata);
//println!("{}",request);
    // Send the request
    if postdata.len() <= 198 {// check multiparts
        if let Err(err) = stream.write_all(request.as_bytes()) {
            //return Err(format!("Failed to send request: {}", err));
            resultstring = format!("Failed to send request: {}", err);
            return resultstring;
        }
    }
    else{

        if let Err(err) = stream.write(&request.as_bytes()[0..1024]) {
            //return Err(format!("Failed to send request: {}", err));
            resultstring = format!("Failed to send request: {}", err);
            return resultstring;
        }
        // let mut response = String::new();
        // if let Err(err) = stream.read_to_string(&mut response) {
        //     resultstring = format!("Failed to recv request: {}", err);
        //     return resultstring;
        // }
        // if Nstring::instring(&response, "200 OK"){
            let mut bytefragment = 1024;
            let mut addbytes:usize;
        let lastbyte = request.len() -1;
            loop {
                // if bytefragment + 1024 > postdata.len(){
                //
                // }
            addbytes = &bytefragment + 1024;
            if  addbytes >= lastbyte{
                addbytes = lastbyte.clone();
            }
                let newpart  = request[bytefragment..addbytes].to_string();
                //println!("bytes[{}]/ {}",&bytefragment,&lastbyte);
                if let Err(err) = stream.write(newpart.as_bytes()) {
                    //return Err(format!("Failed to send request: {}", err));
                    resultstring = format!("Failed to send request: {}", err);
                    return resultstring;
                }
                if bytefragment >= lastbyte{
                    break;
                }

                bytefragment = addbytes.clone();
            }

        //}
        //println!("")


    }

    // Read the response
    let mut response = String::new();
    if let Err(err) = stream.read_to_string(&mut response) {
        resultstring = format!("Failed to recv request: {}", err);
        return resultstring;
    }

    let res = split(&response,"\r\n\r\n");
    if res.len() > 1 {
        return res[1].to_string();
    }
    return res[0].to_string();
}
pub fn decode_html_url(url: &str) -> String {
    let entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];

    let mut decoded = String::new();
    let mut xurl = Nstring::replace(&url, "+", " ");
    xurl = Nstring::replace(&xurl, "%0D", "\n");

    let mut iter = xurl.chars().peekable();

    while let Some(ch) = iter.next() {
        if ch == '%' {
            // Check if it's a valid percent-encoded sequence
            if let (Some(h1), Some(h2)) = (iter.next(), iter.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    if &h1.to_string() != "0" && &h2.to_string() != "0" {
                        decoded.push(byte as char);
                    }
                    continue;
                }
            }
        }

        decoded.push(ch);
    }

    for (entity, replacement) in &entities {
        decoded = decoded.replace(entity, replacement);
    }

    decoded
}
pub fn html_encode(s_txt: &str) -> String {
    let entities: [(u32, &str); 246] = [
        (34, "quot"),
        (38, "amp"),
        (39, "apos"),
        (60, "lt"),
        (62, "gt"),
        (160, "nbsp"),
        (161, "iexcl"),
        (162, "cent"),
        (163, "pound"),
        (164, "curren"),
        (165, "yen"),
        (166, "brvbar"),
        (167, "sect"),
        (168, "uml"),
        (169, "copy"),
        (170, "ordf"),
        (171, "laquo"),
        (172, "not"),
        (173, "shy"),
        (174, "reg"),
        (175, "macr"),
        (176, "deg"),
        (177, "plusmn"),
        (180, "acute"),
        (181, "micro"),
        (182, "para"),
        (183, "middot"),
        (184, "cedil"),
        (186, "ordm"),
        (187, "raquo"),
        (191, "iquest"),
        (192, "Agrave"),
        (193, "Aacute"),
        (194, "Acirc"),
        (195, "Atilde"),
        (196, "Auml"),
        (197, "Aring"),
        (198, "AElig"),
        (199, "Ccedil"),
        (200, "Egrave"),
        (201, "Eacute"),
        (202, "Ecirc"),
        (203, "Euml"),
        (204, "Igrave"),
        (205, "Iacute"),
        (206, "Icirc"),
        (207, "Iuml"),
        (208, "ETH"),
        (209, "Ntilde"),
        (210, "Ograve"),
        (211, "Oacute"),
        (212, "Ocirc"),
        (213, "Otilde"),
        (214, "Ouml"),
        (215, "times"),
        (216, "Oslash"),
        (217, "Ugrave"),
        (218, "Uacute"),
        (219, "Ucirc"),
        (220, "Uuml"),
        (221, "Yacute"),
        (222, "THORN"),
        (223, "szlig"),
        (224, "agrave"),
        (225, "aacute"),
        (226, "acirc"),
        (227, "atilde"),
        (228, "auml"),
        (229, "aring"),
        (230, "aelig"),
        (231, "ccedil"),
        (232, "egrave"),
        (233, "eacute"),
        (234, "ecirc"),
        (235, "euml"),
        (236, "igrave"),
        (237, "iacute"),
        (238, "icirc"),
        (239, "iuml"),
        (240, "eth"),
        (241, "ntilde"),
        (242, "ograve"),
        (243, "oacute"),
        (244, "ocirc"),
        (245, "otilde"),
        (246, "ouml"),
        (247, "divide"),
        (248, "oslash"),
        (249, "ugrave"),
        (250, "uacute"),
        (251, "ucirc"),
        (252, "uuml"),
        (253, "yacute"),
        (254, "thorn"),
        (255, "yuml"),
        (338, "OElig"),
        (339, "oelig"),
        (352, "Scaron"),
        (353, "scaron"),
        (376, "Yuml"),
        (402, "fnof"),
        (710, "circ"),
        (732, "tilde"),
        (913, "Alpha"),
        (914, "Beta"),
        (915, "Gamma"),
        (916, "Delta"),
        (917, "Epsilon"),
        (918, "Zeta"),
        (919, "Eta"),
        (920, "Theta"),
        (921, "Iota"),
        (922, "Kappa"),
        (923, "Lambda"),
        (924, "Mu"),
        (925, "Nu"),
        (926, "Xi"),
        (927, "Omicron"),
        (928, "Pi"),
        (929, "Rho"),
        (931, "Sigma"),
        (932, "Tau"),
        (933, "Upsilon"),
        (934, "Phi"),
        (935, "Chi"),
        (936, "Psi"),
        (937, "Omega"),
        (945, "alpha"),
        (946, "beta"),
        (947, "gamma"),
        (948, "delta"),
        (949, "epsilon"),
        (950, "zeta"),
        (951, "eta"),
        (952, "theta"),
        (953, "iota"),
        (954, "kappa"),
        (955, "lambda"),
        (956, "mu"),
        (957, "nu"),
        (958, "xi"),
        (959, "omicron"),
        (960, "pi"),
        (961, "rho"),
        (962, "sigmaf"),
        (963, "sigma"),
        (964, "tau"),
        (965, "upsilon"),
        (966, "phi"),
        (967, "chi"),
        (968, "psi"),
        (969, "omega"),
        (977, "thetasym"),
        (978, "upsih"),
        (982, "piv"),
        (8194, "ensp"),
        (8195, "emsp"),
        (8201, "thinsp"),
        (8204, "zwnj"),
        (8205, "zwj"),
        (8206, "lrm"),
        (8207, "rlm"),
        (8211, "ndash"),
        (8212, "mdash"),
        (8216, "lsquo"),
        (8217, "rsquo"),
        (8218, "sbquo"),
        (8220, "ldquo"),
        (8221, "rdquo"),
        (8222, "bdquo"),
        (8224, "dagger"),
        (8225, "Dagger"),
        (8226, "bull"),
        (8230, "hellip"),
        (8240, "permil"),
        (8242, "prime"),
        (8243, "Prime"),
        (8249, "lsaquo"),
        (8250, "rsaquo"),
        (8254, "oline"),
        (8260, "frasl"),
        (8364, "euro"),
        (8465, "image"),
        (8472, "weierp"),
        (8476, "real"),
        (8482, "trade"),
        (8501, "alefsym"),
        (8592, "larr"),
        (8593, "uarr"),
        (8594, "rarr"),
        (8595, "darr"),
        (8596, "harr"),
        (8629, "crarr"),
        (8656, "lArr"),
        (8657, "uArr"),
        (8658, "rArr"),
        (8659, "dArr"),
        (8660, "hArr"),
        (8704, "forall"),
        (8706, "part"),
        (8707, "exist"),
        (8709, "empty"),
        (8711, "nabla"),
        (8712, "isin"),
        (8713, "notin"),
        (8715, "ni"),
        (8719, "prod"),
        (8721, "sum"),
        (8722, "minus"),
        (8727, "lowast"),
        (8730, "radic"),
        (8733, "prop"),
        (8734, "infin"),
        (8736, "ang"),
        (8743, "and"),
        (8744, "or"),
        (8745, "cap"),
        (8746, "cup"),
        (8747, "int"),
        (8764, "sim"),
        (8773, "cong"),
        (8776, "asymp"),
        (8800, "ne"),
        (8801, "equiv"),
        (8804, "le"),
        (8805, "ge"),
        (8834, "sub"),
        (8835, "sup"),
        (8836, "nsub"),
        (8838, "sube"),
        (8839, "supe"),
        (8853, "oplus"),
        (8855, "otimes"),
        (8869, "perp"),
        (8901, "sdot"),
        (8968, "lceil"),
        (8969, "rceil"),
        (8970, "lfloor"),
        (8971, "rfloor"),
        (9001, "lang"),
        (9002, "rang"),
        (9674, "loz"),
        (9824, "spades"),
        (9827, "clubs"),
        (9829, "hearts"),
        (9830, "diams"),
    ];

    let mut s_txt_encoded = String::new();
    for c in s_txt.chars() {
        let entity = entities.iter().find(|&&(code, _)| code == c as u32);
        if let Some((_, name)) = entity {
            s_txt_encoded.push_str(&format!("&{};", name));
        } else {
            s_txt_encoded.push(c);
        }
    }
    s_txt_encoded
}

pub fn nscript_setparams_handleconnections(args: &Vec<String>, vmap: &mut Varmap) {
    // this function sets parameters when jumping functions. used on htmlserver
    // because of the code level these params are differently set then functions.
    let id = args.len();
    if id > 0 {
        //println!("codelevle = {}",&vmap.codelevel);
        let codelevelabove = &vmap.codelevel + 0; // <- yeah seems neccesary for vmap.
        for r in 0..id {
            //let paramx = &r + 1;
            let paramid = r + 1;
            let pname = "".to_owned() + "internalparam" + &paramid.to_string();
            //let pname = "param".to_owned() + &paramid.to_string();

            vmap.setvar(pname, &args[r]); // set all param arguments
        }
    }
}

pub fn handle_connection(mut stream: TcpStream, vmap: &mut Varmap) {
    // this is the webserver part it will take a GET request and handle it.
    // text files are on the main thread for other downloads it goes to a other thread
    // .nc files are being regonised and they will return their return results to the user browser.
    // --------------------------------------------------------------------------------------
    let mut buffer = [0; 1024];
    //stream.read(&mut buffer).unwrap();

    match stream.read(&mut buffer) {
        Ok(_) => {
            // procceed the connection.
        }
        Err(_) => {
            // handle OS error on connection-reset
            println!("stream read error ! ");
            return;
        }
    }
    let request = String::from_utf8_lossy(&buffer[..]);
    vmap.setvar("server.request".to_owned(), &request);
    if Nstring::instring(&request, "B blob data") {
        println!("(debug->returning) Blob data entering: {}", &request);
        return; // prevent errors , return!
    }
    if Nstring::instring(&request, "POST") == false && Nstring::instring(&request, "GET") == false {
        println!("A non POST nor GET packet entered: \n {}", &request);
        return; // clearly we aint gonna handle this (yet)
    }
    //println!("req:{}",&request);
    //let request_clone = request.clone();
    let domainname = Nstring::replace(
        &Nstring::stringbetween(&request, "Host: ", "\r\n"),
        "www.",
        "",
    );
    let domainname = split(&domainname, ":")[0];
    vmap.setvar("___domainname".to_owned(), &domainname);
    let request_parts: Vec<&str> = request.split(" ").collect();
    //if request_parts[0] != "GET" {return;} // debugger to find that damn crash on b blobdata.
    let mut pathparts = Vec::new();
    let trimmedreq: String;
    if request_parts.len() > 1 {
        if request_parts[1].contains("B blob data") {
            println!("blobdatafound returning");
            return; // Ignore blob data and return without processing
        }
        trimmedreq = Nstring::trimleft(&request_parts[1], 1);
        pathparts = split(&trimmedreq, "?");
    } else {
        pathparts.push("");
    }
    if pathparts[0] == "" {
        pathparts[0] = "index.nc";
    }

    let mut url_args = Vec::new();
    if pathparts.len() > 1 {
        url_args = split(pathparts[1], "&");
    }

    let mut newparams: Vec<String> = Vec::new();

    for i in 1..10 {
        if url_args.len() > i - 1 {
            newparams.push(decode_html_url(&url_args[i - 1].to_owned()));
        } else {
            newparams.push(String::from(""));
        }
    }

    nscript_setparams_handleconnections(&newparams, vmap);
    let mut webroot = nscript_checkvar("server.serverroot", vmap);
    if webroot == "" {
        webroot = NC_SCRIPT_DIR.to_owned();
    }

    let mut file_path = Nstring::replace(
        &format!("{}{}{}", &webroot, "/public/", &pathparts[0]),
        "/..",
        "",
    );
    let checkthis = webroot.clone() + "domains/" + &domainname + "/http.nc";
    if Nfile::checkexists(&checkthis) {
        file_path = webroot.clone() + "domains/" + &domainname + "/public/" + &pathparts[0];
    }
    //println!("entree:{}",&file_path);
    if request_parts[0] == "POST" {
        let mut postdata = String::new();

        let strippostdata = split(&request, "\r\n\r\n");
        if strippostdata.len() > 1 {
            postdata = "".to_owned() + strippostdata[1]; // used for post buffer data
                                                         //println!("strippedpostdata:{}",&postdata);
        } else {
            //println!("somejacked up stuff");
            return; //some jacked up post request being done.
        }

        if let Some(extension) = Path::new(&file_path)
            .extension()
            .and_then(|os_str| os_str.to_str().map(|s| s.to_owned()))
        {
            if ["nc"].contains(&extension.as_str()) {
                //println!("Its a Post to Nc");
                let bsize = nscript_f64(
                    &Nstring::stringbetween(&request, "Content-Length: ", "Cache").trim(),
                );

                //println!("receiving:{}",&bsize);
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        //return;
                    }
                }
                // match stream.set_read_timeout(Some(Duration::new(0, 10000000))){
                //
                //     Ok(_) => {},
                //     Err(_) => println!("[nctcphttp] Error setting the stream read timeout"),
                // }
                //
                // // let err = result.unwrap_err();
                // // assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
                // match stream.set_write_timeout(Some(Duration::new(0, 10000000))){
                //
                //     Ok(_) => {},
                //     Err(_) => println!("[nctcphttp] Error setting the stream write timeout"),
                // }

                if bsize > nscript_f64(&nscript_checkvar("server.POSTbytesmax", vmap)) {
                    let response = "SERVERERROR:PostSizeExceedsLimit";
                    match stream.write(response.as_bytes()) {
                        Ok(_) => {
                            return;
                        }
                        Err(_) => {
                            return;
                        }
                    }
                }
                //println!("bytesize:{}",&bsize);
                if bsize > 198.0 {
                    // this will make sure this loop will break if something weird happends it
                    // hangs here so this timer (should) solve the issue
                    //let mut  dctimer = Ntimer::init();
                    // set ensurances to break the connection if some hangs.

                    // let err = result.unwrap_err();
                    // assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
                    let mut start_time = Instant::now();
                    loop {
                        let end = Instant::now();
                        if (start_time - end).as_millis() >= 1000 {
                            // dc timer for inactivity should break the loop.
                            //
                            cwrite("closed by timeout", "r");
                            break;
                        }

                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                //println!("\nbytesRead!{}\n",bytes_read);

                                postdata =
                                    postdata + &String::from_utf8_lossy(&buffer[0..bytes_read]);
                                if bytes_read <= 1023 {
                                    break;
                                }

                                // reset the timer.
                                //dctimer = Ntimer::init();

                                start_time = Instant::now();
                                // procceed the connection.
                            }
                            Err(e) => {
                                print!("error nchttp {}", e); // handle OS error on connection-reset)
                                break;
                            }
                        }
                    }
                }
                let strippostdata = split(&postdata, "\r\n\r\n");
                if strippostdata.len() > 1 {
                    postdata = "".to_owned() + &Nstring::replace(&strippostdata[1], "\0", "");
                    // used for post buffer data
                    //println!("strippedpostdata:{}",&postdata);
                }
                vmap.setvar(
                    "POSTDATA".to_owned(),
                    &Nstring::replace(&postdata, "\0", ""),
                );
                // let url_args = split(&postdata, "&");
                // let mut newparams: Vec<String> = Vec::new();
                //
                // for i in 1..10 {
                //     if url_args.len()  > i - 1 {
                //         newparams.push(decode_html_url(&url_args[i-1].to_owned()));
                //     }
                //     else {
                //         newparams.push(String::from(""));
                //     }
                // }
                // nscript_setparams_handleconnections(&newparams,vmap);
                //println!("filepath:{}",&file_path);
                let scriptcode = read_file_utf8(&file_path);
                //println!("script:{}",scriptcode);
                //let compcode = nscript_formatsheet(&scriptcode,vmap);
                let response = nscript_parsesheet(
                    &nscript_replaceparams(&nscript_stringextract(&scriptcode), "param"),
                    vmap,
                );
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        //println!("writingback bytes : {}",bytes_written);
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        //return;
                    }
                }
                //println!("post: {}",postdata);
            }
        }
        return;
    }
    if request_parts[0] == "GET" {
        if let Some(extension) = Path::new(&file_path)
            .extension()
            .and_then(|os_str| os_str.to_str().map(|s| s.to_owned()))
        {
            if ["nc"].contains(&extension.as_str()) {
                let _ = match File::open(&file_path) {
                    Ok(_) => {}
                    Err(_) => {
                        let mut response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                        let nc404file =
                            webroot.clone() + "domains/" + &domainname + "/public/404.nc";
                        println!("404={},", nc404file);
                        if Nfile::checkexists(&nc404file) {
                            //let compcode = nscript_formatsheet(&read_file_utf8(&nc404file),vmap);
                            let compcode = read_file_utf8(&nc404file);
                            let ret = nscript_parsesheet(
                                &nscript_replaceparams(&compcode, "param"),
                                vmap,
                            ); // <-- enables param usage param1 param2 etc.
                            nscript_clearparams_handleconnections(vmap);
                            response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                                "text/html",
                                &ret.len()
                            );
                            stream.write(response.as_bytes()).unwrap();
                            match stream.write(ret.as_bytes()) {
                                Ok(bytes_written) => {
                                    // Check if all bytes were successfully written.
                                    if bytes_written < response.len() {
                                        // Handle the situation where not all data was written if needed.
                                    }
                                }
                                Err(_) => {
                                    return;
                                }
                            }
                            return;
                        } else {
                            stream.write(response.as_bytes()).unwrap();
                            return;
                        }
                    }
                };
               let scriptcode = read_file_utf8(&file_path);
                let oldscriptname = vmap.currentscriptname.clone();
                //            vmap.currentscriptname = file_path.clone();

                //              vmap.parsinglevel = vmap.parsinglevel + 1;
                //vmap.scopecounter = 0;
                let compcode = nscript_stringextract(&scriptcode);
                let ret = nscript_parsesheet(&nscript_replaceparams(&compcode,"param"), vmap);// <-- enables param usage param1 param2 etc.
                //let ret =
                  //  nscript_execute_script(&file_path, "", "", "", "", "", "", "", "", "", vmap);
                nscript_clearparams_handleconnections(vmap);
                vmap.currentscriptname = oldscriptname;
                //vmap.parsinglevel = vmap.parsinglevel - 1;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    "text/html",
                    &ret.len()
                );
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
                match stream.write(ret.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
                return;
            }
            let file_path_clone = file_path.clone(); // clone file_path
            thread::spawn(move || {
                let mut file = match File::open(&file_path_clone) {
                    Ok(file) => file,
                    Err(_) => {
                        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                        stream.write(response.as_bytes()).unwrap();
                        return;
                    }
                };
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).unwrap();
                let content_type = match extension.as_str() {
                    "jpg" | "jpeg" => "image/jpeg",
                    "png" => "image/png",
                    "gif" => "image/gif",
                    "js" => "application/javascript",
                    "txt" => "text/plain",
                    "html" => "text/html",
                    "css" => "text/css",
                    _ => "application/octet-stream",
                };
                let response = format!(
                    "HTTP/2.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    content_type,
                    contents.len()
                );
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            eprintln!("Not all data was written to the stream.");
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(error) => {
                        return;
                    }
                }
                match stream.write(&contents) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < contents.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            });
            return;
        }
    }
}
pub fn ncwebserver(vmap: &mut Varmap) -> std::io::Result<()> {
    //let args: Vec<String> = env::args().collect();

    // The first argument (index 0) is the name of the binary itself.
    // The actual command-line arguments start from index 1.
    // if args.len() > 1 {
    //     println!("Command-line arguments:");
    //     for (index, arg) in args.iter().enumerate().skip(1) {
    //         println!("{}: {}", index, arg);
    //     }
    // } else {
    //     println!("No command-line arguments provided.");
    // }

    //let mut vmap = Varmap::new(); // global

    println!("Starting Nscript WebServer {}", NSCRIPT_VERSION);
    println!("____________________________________");
    let mut webroot = nscript_checkvar("server.serverroot", vmap);
    if webroot == "" {
        webroot = NC_SCRIPT_DIR.to_owned();
    }
    // run Nscript:server.nc ,define pre logic here, this runs before the stream starts.
    vmap.setvar("self".to_owned(), "server"); //<- set self in nscript during scope
    let serverscriptfilename = webroot.clone() + "system/webserver.nc";
    nscript_execute_script(
        &serverscriptfilename,
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        vmap,
    );
    // retrieve the prop's set for class server in nscript:server.nc
    let server_addres_nc = nscript_checkvar("server.ip", vmap);
    let server_port_nc = nscript_checkvar("server.port", vmap);

    let listener: TcpListener;
    if server_port_nc != "" && server_addres_nc != "" {
        // when the server.nc is run class server.ip and server.port be checked!
        listener = TcpListener::bind(format!("{}:{}", &server_addres_nc, &server_port_nc)).unwrap();
        println!(
            "Server started at http://{}:{}",
            &server_addres_nc, &server_port_nc
        );
    } else {
        // if missing serverclass or something, use the constants
        listener = TcpListener::bind(format!("{}:{}", NC_SERVER_ADDRESS, NC_SERVER_PORT)).unwrap();
        println!(
            "Server started at http://{}:{}",
            NC_SERVER_ADDRESS, NC_SERVER_PORT
        );
    }
    // sets the
    // acceptsocketlisterns to continue and not hold on the script
    #[cfg(windows)]
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    #[cfg(not(windows))]
    listener.set_nonblocking(true)?;

    // this checks your /domains/ folder for subfolders
    // you can name a folder to your dns-domain
    // all http to this domain be rerouted to this folders

    let domaindir = webroot.clone() + "domains/";
    println!("domaindir={}", &domaindir);
    let domdir = Nfile::dirtolist(&domaindir, false);
    let domaindirarr = split(&domdir, NC_ARRAY_DELIM);
    for domainscript in domaindirarr {
        if domainscript != "" {
            vmap.setvar("___domainname".to_owned(), &domainscript);
            let domainscript = webroot.clone() + "domains/" + domainscript.trim() + "/http.nc";
            nscript_execute_script(&domainscript, "", "", "", "", "", "", "", "", "", vmap);
            println!("Loading domain script:[{}]", &domainscript);
        }
    }
    println!("Domains loaded, starting listener");

    loop {
        nscript_loops(vmap);
        match listener.accept() {
            Ok((stream, _)) => {
                match stream.peer_addr() {
                    Ok(res) => {
                        let remote_ip = res.ip();
                        vmap.setvar("___thissocketip".to_owned(), &remote_ip.to_string());
                        let onconfunc =
                            "server.onconnect(\"".to_owned() + &remote_ip.to_string() + "\")";
                        nscript_checkvar(&onconfunc, vmap);
                        handle_connection(stream, vmap);
                        //println!("connection ok and closed");
                    }
                    Err(err) => {
                        println!("Connection error{}", err);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No incoming connections yet,
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

pub fn get_http_file_content(
    host: &str,
    port: u16,
    path: &str,
    pathoutput: &str,
) -> Result<Vec<u8>, std::io::Error> {
    let mut stream = TcpStream::connect((host, port))?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    // Find the position of the double CRLF (indicating the end of headers)
    if let Some(index) = response.windows(4).position(|window| window == b"\r\n\r\n") {
        // Skip the headers and extract the content
        let content = response.split_off(index + 4);
        let mut file = File::create(pathoutput)?;

        // Write the binary data to the file
        file.write_all(&content)?;
        return Ok(content);
    }

    // If double CRLF not found, return the full response (including headers)
    Ok(response)
}

pub fn get_http_content(host: &str, port: u16, path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut stream = TcpStream::connect((host, port))?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
        path, port, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    // Find the position of the double CRLF (indicating the end of headers)
    if let Some(index) = response.windows(4).position(|window| window == b"\r\n\r\n") {
        // Skip the headers and extract the content
        let content = response.split_off(index + 4);
        return Ok(content);
    }

    // If double CRLF not found, return the full response (including headers)
    Ok(response)
}
// fn handlepost(){// <------------------------------------ Need to be worked in handle connection
// still.
//
//      if request_parts[0] == "POST" {
//         let mut stream_clone = stream.try_clone().unwrap();
//         // Check if the file path has the ".nc" extension
//         if let Some(extension) = Path::new(&file_path).extension().and_then(|os_str| os_str.to_str().map(|s| s.to_owned())) {
//             if extension == "nc" {
//                 // Spawn a new thread to handle the file upload
//                 let request_clone = request.clone();
//                 let file_path_clone = file_path.clone();
//                 thread::spawn(move || {
//                     // Read the request body (file data) until the end
//                     let mut boundary = String::new();
//                     if Nstring::stringbetween(&request_clone, "boundary=", "\r\n") != "" {
//                         let boundary_str = &Nstring::stringbetween(&request_clone, "boundary=", "\r\n");
//                         boundary = boundary_str.to_owned();
//
//                     }
//
//                     // Process each part of the multipart form data
//                     let mut part_start_delimiter = format!("\r\n--{}\r\n", &boundary).into_bytes();
//                     let mut part_end_delimiter = format!("\r\n--{}--\r\n", &boundary).into_bytes();
//
//                     let mut bufferupload = [0; 1024];
//                     let mut received_data = Vec::new();
//
//                     loop {
//                         let bytes_read = stream_clone.read(&mut bufferupload).unwrap();
//                         if bytes_read == 0 {
//                             break;
//                         }
//
//                         received_data.extend_from_slice(&bufferupload[..bytes_read]);
//
//                         // Check if the received data contains the part start or end delimiter
//                         if received_data.windows(part_start_delimiter.len()).any(|window| window == &part_start_delimiter[..])
//                         || received_data.windows(part_end_delimiter.len()).any(|window| window == &part_end_delimiter[..]) {
//                             // Process the part (file data)
//                             let part_data = received_data.clone();
//
//                             // Extract necessary information from the part data and perform custom logic
//                             let content_type = Nstring::stringbetween(&request_clone, "Content-Type: ", "\r\n");
//                             let filename = Nstring::stringbetween(&request_clone, "filename=\"", "\"");
//
//                             // Write the file data to a specific directory
//                             let output_dir = "./testfiles/uploads";
//                             let output_path = format!("{}/{}", output_dir, filename);
//                             let mut file = File::create(output_path).unwrap();
//                             file.write_all(&part_data).unwrap();
//
//                             // Perform additional logic specific to MP3 files
//                             // For example, you can extract metadata, process audio, etc.
//
//                             // Send the response back to the client
//                             let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n", "text/plain");
//                             let message = "File upload for .nc file completed.";
//                             let response = format!("{}{}", response, message);
//                             stream_clone.write(response.as_bytes()).unwrap();
//                             return;
//                         }
//                     }
//                 });
//                 return;
//             }
//         }
//     }
// }
