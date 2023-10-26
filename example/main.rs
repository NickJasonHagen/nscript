
extern crate nscript_lib;
use nscript_lib::*;


fn yourfunctionmapping(vmap: &mut Varmap)-> String{
    // testoverride requires vmap, this function extents the parsers functions to be used in Nscript.
    // you can retrieve the nscript call's data by using : vmap.funcname ( the name of the function)
    // and vmap.param1 ~ vmap.param9 , hardcoded functions be capped to 9 arguments, here you can
    // map your own logic, just return the result as a String and the parser will manage the rest.
    // params and funcnames are all String. if they are unused by nscript they are set to be empty
    // if your function requires data you can check by if vmap.param1 != "" error(yourlogic)

    // example: if nscript calls : testing("first arg given","2","third")

    // map your functions and their logics inside this match
    // returns a string back to nscript
    match vmap.funcname.as_str() {
        "testing" => { // maps this scope as function testing() in nscript.
            cwrite("testingoverrides!!","g");

            cwrite(&vmap.param1,"g"); // <- holds "first arg given"
            cwrite(&vmap.param2,"g");
            cwrite(&vmap.param3,"g");
            return vmap.param1.to_owned()
        }
        "secondmapping" => {
            // requiring arguments. (nscript has overrides and defaults for each
            // so you need to check and catch your own things for it, like below!)
            if vmap.param1 != ""{
                cwrite("well this only executes if param1 was given","y");
                return "somethingtoreturn".to_owned()
            }
            else{
                cwrite("ohhh something dind happen! the func did not give argument1..","");
                // if you want to exit or not is up to you
                // but you would do it here.
            }
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
    vmap.setextentionfunctions(yourfunctionmapping);

    // this begins the nscript engine1, we set a init script and run it
    let initscript = SCRIPT_DIR.to_owned() +"init.nc";
    nscript_execute_script(&initscript,"","","","","","","","","",&mut vmap);

    loop {
        // this handles the nscripts loop system.
        nscript_loops(&mut vmap);

    }
}

