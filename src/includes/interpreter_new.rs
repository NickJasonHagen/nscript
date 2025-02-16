use crate::*;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
//use media::Media;
//extern crate id3;
use std::thread;
//use super::nscript_sound;
//use std::path::PathBuf;
pub const NC_PROGRAM_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub const NC_ARRAY_DELIM: &str = "  }~{ ";
pub const NC_ASYNC_LOOPS_KEY: &str = "coroutine"; // async loops scopes keyword
use dirs;
pub type NscriptCustomFunctions = fn(&mut Nscript) -> String;

pub struct Nscript{
    interpreter: Interpreter,

}
impl Nscript {
    pub fn new() -> Nscript{
        Self{
            interpreter: Interpreter::new(),

        }
    }
    pub fn get_var(&mut self,varname:&str)->String{
        self.interpreter.variables.evaluate(varname)
    }
}


struct Interpreter{
    lexer: Lexer,
    variables: Variables,
    classes: HashMap<String,Classes>,
    codelevel:u16,
}

impl Interpreter {
    pub fn new() -> Interpreter{
        Self{
            lexer: Lexer::new(),
            variables: Variables::new(),
            classes: HashMap::new(),
            codelevel: 0,
        }
    }


}
struct Lexer{
    pub codeblockmap: HashMap<String, String>,
    pub codeblockvectormap: HashMap<String, Vec<Vec<String>>>,
}

impl Lexer{
    pub fn new() -> Lexer{
        Self{
            codeblockmap: HashMap::new(),
            codeblockvectormap: HashMap::new(),
        }
    }
     fn setcode(&mut self, name: &str, code: &str) {
        self.codeblockmap.insert(name.to_string(), code.to_owned());
    }
     fn setcodevector(&mut self, name: &str, code: &str) {
        let mut codearray: Vec<Vec<String>> = Vec::new();
        let linearray: Vec<String> = code.split("\n").map(|s| s.to_string()).collect();
        for line in &linearray{
            let wordvec = line.split(" ").map(|s| s.to_string()).collect();
            codearray.push(wordvec);
        }
        self.codeblockvectormap.insert(name.to_owned(),codearray);
    }
     fn getcodevector(&mut self, name: &str) -> Vec<Vec<String>> {
        let g = self.codeblockvectormap.get_key_value(name);
        let result = match g {
            None => {
                Vec::new()
            }
            Some((_i, k)) => {
                let result = k.to_owned();
                result
            }
        };
        result
    }
     fn getcode(&mut self, name: &str) -> String {
        let g = self.codeblockmap.get_key_value(name);
        let result = match g {
            None => {
                String::from("")
            }
            Some((_i, k)) => {
                let result = k.to_owned();
                result
            }
        };
        result
    }
    fn stripcomments(&mut self, coderaw: &str) -> String {
        // strips off all comments per lines.
        let lines = coderaw.split("\n");
        let mut newcode = String::new();
        for line in lines {
            if line != "" {
                newcode = newcode + &split(&line,"//")[0].trim() + "\n";
            }
        }
        newcode
    }
pub fn nscript_stringextract(text: &str) -> String {
    // this will convert all static strings to a ^hexnumber
    // ive used this so that the spaces woulnd interfere with the syntax.
    // nscript_checkvar() will regonise ^3131 formats and unhex them where needed.
    // ------------------------------------------------------------------------
    let mut parsingtext = replace(&text.to_string(), "\\\"", "#!@NSCRIPTQUOTE#@!");
    parsingtext = replace(&parsingtext, "\"\"", "@emptystring");
    loop {
        let splitstr = Nstring::stringbetween(&parsingtext, "\"", "\"");
        if splitstr != "" {
            let packed = "^".to_owned()
                + &string_to_hex(&Nstring::replace(&splitstr, "#!@NSCRIPTQUOTE#@!", "\" "));
            let toreplace = "\"".to_owned() + &splitstr + "\"";
            parsingtext = Nstring::replace(&parsingtext, &toreplace, &packed);
        } else {
            break;
        }
    }
    parsingtext
}
}

struct Variables {
  varmap: HashMap<String, String>,
}

impl Variables {
    pub fn new() -> Variables{
        Self{
            varmap: HashMap::new(),
        }
    }
    fn evaluate(&mut self,varname:&str) ->String{
            let g = self.varmap.get_key_value(varname);
            match g {
                None => {
                    String::new()
                }
                Some((_i, k)) => k.to_owned(),
            }
    }
    fn assign(&mut self,varname:&str,data:&str){
        self.varmap.insert(varname.to_string(),data.to_string());
    }
}

struct Classes {
    index: Vec<String>,
    functions: Vec<String>,
    parents: Vec<String>,
    children:Vec<String>,
    properties: HashMap<String,String>,
}

impl Classes {
    pub fn new() -> Classes{
        Self {
            index: Vec::new(),
            functions: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            properties: HashMap::new(),
        }
    }
    fn set_property(&mut self ,varname:&str,data:&str){
        if self.get_property(&varname)  == String::new() {
            self.index.push(varname.to_string());
        }
        self.properties.insert(varname.to_string(),data.to_string());
    }
    fn get_property(&mut self,varname:&str) ->String{
            let g = self.properties.get_key_value(varname);
            match g {
                None => {
                    String::new()
                }
                Some((_i, k)) => k.to_owned(),
            }
    }
    fn delete_property(&mut self,varname:&str){
        self.index.retain(|x| x != varname);
        self.properties.remove(varname);
    }
}

fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    // returns a str array vector
    let r: Vec<&str> = s.split(p).collect();
    return r;
}

fn replace(s: &str, f: &str, r: &str) -> String {
        if f == "" || s == ""{
            //println!("debugger cannot replace none?{} by none?{} ",&s,&f);
            return s.to_string();
        }
        // i know slaat nergens op.. :P
        return s.replace(f, r);
    }
