use crate::*;


pub fn pooladd(pool: &str,entree: &str) -> String{
    // nscript arrays wich work with unique entrees,
    // adding some thats already there wont be added.
    if entree == "" {return pool.to_string();}
    let array = split(&pool,NC_ARRAY_DELIM);
    let mut newstring = String::new();
    let mut found = false ;
    for entr in array{
        if entr == entree {
            found = true;
        }
        newstring = newstring + &entr + NC_ARRAY_DELIM;
    }
    if found == false{
        newstring = newstring + &entree + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&newstring,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        newstring = Nstring::trimright(&newstring, NC_ARRAY_DELIM.len());
    }
    if Nstring::fromleft(&newstring,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        newstring = Nstring::trimleft(&newstring, NC_ARRAY_DELIM.len());
    }
    newstring
}

pub fn poolremove(pool: &str,entree: &str)-> String{

    let array = split(&pool,NC_ARRAY_DELIM);
    let mut newstring = String::new();
    for entr in array{
        if entr != entree && entr != "" {
            newstring = newstring + &entr + NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&newstring,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        newstring = Nstring::trimright(&newstring, NC_ARRAY_DELIM.len());
    }
    newstring
}

pub fn inpool(pool: &str,entree: &str) -> String{
    // nscript arrays wich work with unique entrees,
    // this function checks wheter a id is inside the pool returns 1 if found 0 if not
    let array = split(&pool,NC_ARRAY_DELIM);
    for entr in array{
        if entr == entree {
            return "1".to_string();
        }
     }

        return "0".to_string();

}

pub fn arraypush(array: &str,data: &str ) -> String {
    return "".to_owned() + &array + NC_ARRAY_DELIM + &data
}

pub fn arraypushroll(array: &str,data: &str ) -> String {
    let splitsel = split(&array,NC_ARRAY_DELIM)[0];
    let striplen = splitsel.len() + NC_ARRAY_DELIM.len();
    let newarr = "".to_owned() + &array + NC_ARRAY_DELIM + &data;
    return Nstring::trimleft(&newarr,striplen);
}

pub fn arrayfilter(array: &str,tofilter: &str) -> String {
    let mut ret = String::new();
    for entree in split(&array,&NC_ARRAY_DELIM) {
        if Nstring::instring(&entree, &tofilter) == false {
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}

pub fn arraysort(array: &str) -> String {
    let mut strings = split(&array,&NC_ARRAY_DELIM);
    strings.sort();
    let mut ret = String::new();
    for each in strings {
        ret = ret + &each + &NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    } else {
        ret
    }
}

pub fn arraysearch(array: &str,tosearch: &str) -> String{
    //println!("searching array:{} for {}",&array,&tosearch);
    let mut ret = String::new();
    for entree in split(&array,&NC_ARRAY_DELIM){
        if Nstring::instring(&entree, &tosearch){
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}
pub fn arrayreverse(array: &str) -> String{
    //println!("searching array:{} for {}",&array,&tosearch);
    let mut ret = String::new();
    for entree in split(&array.reverse(),&NC_ARRAY_DELIM){
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
     }
    if Nstring::fromleft(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimleft(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}
pub fn arrayshuffle(arraystr:&str) -> String{
    let mut array = split(&arraystr,NC_ARRAY_DELIM);
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
    let mut ret = String::new();
    for entrees in array{
        ret = ret + &entrees + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    } else {
        ret
    }

}

pub fn arrayfirstout(array:&str) -> String{
    let  array = split(&array,NC_ARRAY_DELIM);
    let mut ret = String::new();
    let u = array.len();
    for each in array{
        if u > 0{
            ret = ret + &each + NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    ret
}

pub fn arraylastout(array:&str) -> String{
    let  array = split(&array,NC_ARRAY_DELIM);
    let mut ret = String::new();
    let u = array.len();
    for each in array{
        if u > 0{
            ret = ret + &each + NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    ret
}

pub fn arraystartwith(array: &str,tosearch: &str) -> String{
    //println!("searching array:{} for {}",&array,&tosearch);
    let mut ret = String::new();
    let chrs = tosearch.len();
    for entree in split(&array,&NC_ARRAY_DELIM){
        if Nstring::instring(&entree[0..chrs], &tosearch){
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}

pub fn arrayendwith(array: &str,tosearch: &str) -> String{
    //println!("searching array:{} for {}",&array,&tosearch);
    let mut ret = String::new();
    let chrs = tosearch.len();
        for entree in split(&array,&NC_ARRAY_DELIM){
let end = entree.len();

        if Nstring::instring(&entree[end-chrs..end], &tosearch){
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}
pub fn identifierarray(name:&str,i:&str,fromi:&str) -> String{
    let range = nscript_i32(i) as usize;

    let mut rangefrom = nscript_i32(fromi) as usize;
    let mut returnstring = String::new();
    if rangefrom == 0 {
        rangefrom = 1;
    }
    for x in rangefrom..range+1{
        returnstring = returnstring + &name + &x.to_string() + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&returnstring, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&returnstring, NC_ARRAY_DELIM.len());
    }
    returnstring
}

pub fn array_retain(array: Vec<String>, to_remove: &str) -> Vec<String> {
    let mut ret = array;
    ret.retain(|x| x != to_remove);
    ret
}

pub fn array_contains(array: &[String], to_check: &str) -> bool {
    array.contains(&to_check.to_string())
}
pub fn array_contains_all(main_array: &[String], sub_array: &[String]) -> bool {
    sub_array.iter().all(|item| main_array.contains(item))
}
