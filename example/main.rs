
extern crate nscript;
use nscript::*;

fn helloworld(param1:&str)->String{
    return "helloworld! hi ".to_owned() + &param1;
}
fn examplefunction2(param1:&str)->String{
    return "and more ! helloworld! hi ".to_owned() + &param1;
}

fn mycustombindings(vmap: &mut Varmap)-> String{
    // testoverride requires vmap, this function extents the parsers functions to be used in Nscript.
    // you can retrieve the nscript call's data by using : vmap.funcname ( the name of the function)
    // and vmap.param1 ~ vmap.param9 , hardcoded functions be capped to 9 arguments, here you can
    // map your own logic, just return the result as a String and the parser will manage the rest.
    // params and funcnames are all String. if they are unused by nscript they are set to be empty
    // if your function requires data you can check by if vmap.param1 != "" error(yourlogic)

    // example: if nscript calls : testing("first arg given","2","third")

    // map your functions and their logics inside this match
    // returns a string back to nscript

    // ownerships of the arguments applied to the nscript call made
    // yes these are 1 to 9
    let param1 = vmap.param1.to_owned();
    let param2 = vmap.param2.to_owned();
    let param3 = vmap.param3.to_owned();
    let param4 = vmap.param4.to_owned();
    let param5 = vmap.param5.to_owned();
    let param6 = vmap.param6.to_owned();
    let param7 = vmap.param7.to_owned();
    let param8 = vmap.param8.to_owned();
    let param9 = vmap.param9.to_owned();
    // ! make sure to return something! Or the core functions will also be checked for a match if the
    // empty string is returned.
    match vmap.funcname.as_str() {
        "helloworld" => { // maps this scope as function testing() in nscript.
            return helloworld(&param1);
        }
        "examplefunction2" => {
            return examplefunction2(&param1);
        }
        _ =>{
        }
    }
    "".to_owned() // if no match continue core mappings.

}

fn main() -> std::io::Result<()>  {

    //vmap is the hashmap which stores all data for nscript so keep it safe.
    let mut vmap = Varmap::new(); // global

    // here we inject the function parser with your functions
    vmap.setextentionfunctions(mycustombindings);

    // this begins the nscript engine1, we set a init script and run it
    let initscript = NC_SCRIPT_DIR.to_owned() +"init.nc";
    nscript_execute_script(&initscript,"","","","","","","","","",&mut vmap);

    loop {
        // this handles the nscripts loop system.
        nscript_loops(&mut vmap);

    }
}

