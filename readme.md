#integrate nscript into your rust projects
```rust

extern crate nscript;
use nscript::*;


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

```
#Nscript examples:
## /elseif :nscript core function
```swift
// else if statements 


var = 4
// if this is false the elseif statements be checked until one of them is true,
// the other elseifs be skipped.
if var > 5 { 
    print("..")
}
elseif var > 1 {
    print("var bigger 1 yes","green") // <-- this will be printed
}
elseif var > 2 {
    print("var bigger 2") //<-- this wont
}   

```
## /sleep :nscript core function
```swift
// this function holds the script for a duration in miliseconds
// it can be used in loops to reduce the powerusage.

while 1 {
    sleep(10)
}

```
## /helloworld :nscript core function
```swift
// print has 2 arguments first is your msg
// the second can be a color as a string, "green", red ,blue , yellow
print("helloWorld","green")


```
## /threads :nscript core function
```swift
class tcp{
    func client(ip,port){
        
        tmr = timerinit()
        loop{
            self.serversocket = tcpconnect(ip,port)
            if self.serversocket != "" {
                print(cat("connected succesfully:",self.serversocket),"p")
                break
            }
            if timerdiff(tmr) > 999 {
                print("timed-out")
                tmr = timerinit()
            }
        }
        print(self.serversocket,"r")

    }
    func server(ip,port){
        self.listenersocked = tcplistener(ip,port)
        coroutine self.listenersocked{
            
            incsocket = tcpaccept(self)
            
            if incsocket != ""{

                print(cat("new socket connected:",incsocket),"p")
                coroutine incsocket{
                    res = tcpreceive(self)
                    if res != ""{
                        print(cat("socketloop:",self,"inc:[",res,"]"),"y")
                        tcpsend(self,"alrighty")
                    }                  
                }
            }
        }
        return self.listenersocked
    }
}
thread [c:tcp]{
    sleep(2)
    tcp.client("127.0.0.1",8888)
    timer = timerinit()
    coroutine "x"{
        if timerdiff(timer) > 999{
            tcpsend(tcp.serversocket,cat("t1=",@sec,",haii"))
            timer = timerinit()
            
        }

    }
}    
thread [c:tcp]{
    sleep(2)
    tcp.client("127.0.0.1",8888)
    timer = timerinit()
    coroutine "x"{
        if timerdiff(timer) > 679{
            tcpsend(tcp.serversocket,cat("t2=",@sec,",oiii"))
            timer = timerinit()
            
        }

    }
}   


tcp.server("127.0.0.1",8888)

```
## /arrayfilter :nscript core function
```swift
//array filter will create a new array based on the given array, any entree containing the filter substring will be moved out of the new array.


array = ["tom@hotmail.com","gerry@gmail.com","belle@gmail.com","jeffrey@hotmail.com")

nohotmailarray = arrayfilter(array,"@hotmail.com") //<-- filter all hotmails form the array.

```
## /trimleft :nscript core function
```swift
// this allows you to trim a string from the left side by a number of characters,
// it returns the new string

string = "email=big_john@swaggers.com"
newstring = trimleft(string,6)

```
## /discordmsg :nscript core function
```swift
// so this function allows you to sent a msg to your discord channels,
// in discord you need to go to your channel settings to copy the hook api.
// function discordmsg(msg,api)

api = "http://discord.blabla/myapicodeblabla089912390123" 
msg = "Heeeeeeeeeeeeellllllloooooooooo goooooooooooooooood mornnnnnnnnning discord !"
discordmsg(msg,api)

```
## /objfromjson :nscript core function
```swift
// objtojson(obj) - objfromjson(objname,jsonstring)
// with these functions you can convert all properties of a obj/class
// to a json string and load it back in or sent it to other server to load it in,


class myclass{
    self.name = "superclass"
    self.descriptiuon = "im gonna be used for json"
}


// convert a class to json string
jsonstring = objtojson(myclass)

// load a new class name (arg 1) with a jsonstring (arg2) 
// and trow the objref back to the variable.
copiedclass = objfromjson("newuniqueclassreferencestring",jsonstring)

// you can now use the properties on this variable
// it will be performing on class newuniqueclassreferencestring
print(copiedclass.name,"green")

```
## /fromright :nscript core function
```swift
// with this function you get a new string from the left or right side of a stirng by a given number of characters
// fromright(string,charsasInt) fromleft()


string = "music.mp3"
filetype = fromright(string,4)

```
## /arraysearch :nscript core function
```swift
// this function allows you to search in a array for a substring, a new array will return with all entrees wich has the substring in them.

array = ["apple","tree","stair","chair"] // define a array
resultarray = arraysearch(array,"e") // create a new array by searching the array for anything with "e" 

for each in array {
    print(each)
}

```
## /save :nscript core function
```swift
// these are database like system to quickly store or load data to a file ( textformat )
// it saves the header on a line , and the line bellow it contains the data.
// headers must be unique per file, or it overwrites.

save("#header","data","filelocation")
data = load("#header","filelocation")

```
## /years_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /for :nscript core function
```swift
// there are 2 different loops you can use with the for scope
// for in: this
array = ["apple","pear","lemon"]
for x in array{
    print(x) // <--- this loop will do 3 runs , x be apple , then pear then lemon
}
// for to this will set the x to a counter starting at 1 till your number is reached.
for x to 100 {
    print(x) //<--- x is a counter 1 to 100
}

```
## /timerinit :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /loop :nscript core function
```swift
// loop { break} is a scope wich will run until you say break. 
// unlike breaking a async with a reference this loop only needs 1 word

i = 0 
loop {
    i ++
    if i > 10 {
        break    
    }
}

```
## /exit :nscript core function
```swift
// yea i know right gotta love nscript why close it, but in case you wonder how.

exit

```
## /filesizebytes :nscript core function
```swift
// this function returns a full format of the file size 
// related : filesize(fname)

size = filesizebytes("./testfile.bin") // returns the actual size of the file in bytes.


```
## /reflection :nscript core function
```swift
// function/variable reflection/dispatching
// these can be used on functions, class-functions and variable-properties
// if is contains the * symbol itwill be the evaluated data as a string used to identify.
class a {
	func test(){
		print("oi")
	}
	self.info = "bladibla"
}

classname = "a"
functionname = "test"
prop = "info"
*classname.test() // reflecting the classnamepart
a.*functionname() // reflecting the function part
*classname.*functionname()  // reflecting both classname and function
print(a.*prop) // reflecting the property part
print(*classname.*prop) // yeah you get it !

```
## /dirmove :nscript core function
```swift
// if you want to move a directory
// dirmove(source,destination)

status = dirmove("./dir/","./bk/dir/")

```
## /filewrite :nscript core function
```swift
// write data to a file
filewrite("./bangalist.txt","Kim, Rachel, Joyce")

```
## /unzip :nscript core function
```swift
// to unzip a file to a directory you can use this function
// unzip(zipfile,extractionlocation)
// returns status || related zip(dir,zipfile)


status = unzip("./myzip.zip","./extracthere/")

```
## /days_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /variables :nscript core function
```swift
// in nscript variables dont come with a prefix their just words.
// you can set a variabe with functions, strings, numbers, macro's switches.

myvar = "this is a variable made from a string"
myvar = 123
myvar = 0.123
myvar = listdir(@scriptdir)

// to escape the " symbol you can use \" inside a "" string. the interpreter will keep them in your string.
var = "this is a quote -> \" <-"

//multi line variables
// be aware of comments inside this !!
multilinevar = " this is a 
multiline variable // not a comment !! part of the string!
everything between the quotes
with exeptions of the escape \" 
the interpreter will format this
(tech: to 1 line of syntax for interpretation)"

```
## /dircreate :nscript core function
```swift
// this functions creates a directory

dircreate("./newdir/")

```
## /arrays :nscript core function
```swift
// arrays in nscript are still variables( they contain a delimerstring.)
// to define a array
//-- 
// if you call a number outbounds the array returns a empty string ""
// this will not cause a runtime error ( by default)!
//-----------------------------------------------
array = ["1","2","3","4"]

arraysize = array[?] // <-- the questionmark used on [] in a array variable
//results in the size of the array

print(array[1])

```
## /stringtobase64 :nscript core function
```swift
// to encrypt or decrypt a file from binary to a string or from a string to binary 
// you can use a system called base64, ive implemented 2 functions
// base64tostring(base64satring) and stringtobase64(string)

print(base64tostring("QmlnIGpvaG4gbG92ZXMgY29sZCBiZWVycyE=")) // this will decrypt this top most important msg and print it.

//to pack a string
packed = stringtobase64("super secret stuff whooohooo!")

```
## /stackpush :nscript core function
```swift
// stacks, these are last in first out stacks. you can push things to a stack and pop them out one by one.
// stacks have a string reference. the first argument wil represent the name of the stack wich has to be unique, you can enter a variable but it has to hold the reference of the stack's reference as a string.


// define a unique reference
mystack = "mystackreference"

stackpush(mystack,"hearts 4")
stackpush("mystackreference","hearts 8") //<--- here you see the stack can also be used staticly by string.

func stackref(){
    return "mystackreference"
}

stackpush(stackref(),"hearts king") //<-- reffed by a function return 

mycard = stackpop(mystack)

```
## /filedelete :nscript core function
```swift
// filedelete(filename) returns the status

status = filedelete("./googlechrome")

```
## /pooladd :nscript core function
```swift
// pool system, this is the same format as a array! but using pooladd() and poolremove() 
// will make sure that each entree in this array is unique !
// if an entree is pushed twice the array is unchanged
// pooladd(array,entree) , poolremove(array,entree) < - both return the array as result

array = ""
array = pooladd(array,"Joyce")
array = pooladd(array,"Kim")
array = pooladd(array,"Alice")
array = pooladd(array,"David") //<--- now this one is already in the pool and wont show

array = poolremove(array,"David") // <-- gues the ladies dont want David to be in the pool ! 

```
## /file_read_utf8 :nscript core function
```swift
// well as mister Satan likes to mess things up his Satansoft product kinda dont use utf8 format.
// however in some cases ( like interpreteting) you dont want to deal with mister satan.
// so this function allows you to read a file and parse it to utf8 so you can do whatever holyness it may be you like to do

utf8data = file_read_utf8("./satansoftcreatedfile.docx")

```
## /asyncloops :nscript core function
```swift
// async loops these are somewhat like how go routines work.
// the difference is you have to identify a loop by a string.
// this reference is important when you want to break the loop
// the loop can be broken from anywhere as long as you reference it correctly

// Async loops set the self variable to their reference, using properties on this automaticly converts the refernce to an object.
// you can break the loop whitin the scope "Break self" or break it from elsewhere with "break myloopref"

myloopref = "mainloop"

async myloopref{
    break self
}
//this can be done outside of the scope.
break myloopref


```
## /minutes_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /stackpop :nscript core function
```swift
// stacks, these are last in first out stacks. you can push things to a stack and pop them out one by one.
// stacks have a string reference. the first argument wil represent the name of the stack wich has to be unique, you can enter a variable but it has to hold the reference of the stack's reference as a string.


// define a unique reference
mystack = "mystackreference"

stackpush(mystack,"hearts 4")
stackpush("mystackreference","hearts 8") //<--- here you see the stack can also be used staticly by string.

func stackref(){
    return "mystackreference"
}

stackpush(stackref(),"hearts king") //<-- reffed by a function return 

mycard = stackpop(mystack)

```
## /months_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /terminalinput :nscript core function
```swift
// this function can be set to run commands inside the termina, keep in mind that this does pause the script until the input is given
// userinput(message,defaultpromptonEnter)


userinput = terminalinput("Do you want a cold beer ?","Aaaaaii")


```
## /dirdelete :nscript core function
```swift
// if you want to delete a directory

status = dirdelete("./microsoft/")

```
## /ifstatements :nscript core function
```swift
// if statements allow you to perform checks you can nest them as much as you like,

if 1 == 1 {
    print("true")
}
// the else scopes work somewhat different, they act on the last statement so you can kinnda still put code between the if and else scope.. :P well enjoy !
else {
    print("not tru")
}

// you can use and by "and" / "&&"
if 1 == 1 and 2 == 2 {
    print("also true")
}
if 1 == 1 && 2 == 2 {
    print("also true")
}

// you can use or by "or" / "||"
if 1 > 1 or 2 == 2 {
    print("also true")
}
if 1 != 1 || 2 == 2 {
    print("also true")
}

// you can also combine and or and nest them

if user.status == "loggedin" || user == "Big John" {
    if 1 == 1 {
        print("true as shit","green")
    }
}

```
## /switch :nscript core function
```swift
// following shows how to do a match (//switch) like thing

tocheck = "4"

match tocheck{
    "1" | "2" =>{
        print("first")
    }
    "3" => {
        print("second")
    }
    "4" => {
        print("fourth")
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}

// you can also catch a variable by a match
// on the match you can use a scope OR 1 word this be returned 
// to the variable. the last lines result is auto-returned.
myvar = match tocheck{
    "1" | "2" => print("first") //< no scope usage. print returns !
    "3" => "staticstring" //<-- static return
    "4" => {
        if @day == "01" {
            return "first day of the month" //<- use return to breakback
        }
        "not the first day"
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}

```
## /print :nscript core function
```swift
// print has 2 arguments first is your msg
// the second can be a color as a string, "green", red ,blue , yellow
// print returns your printed value so you can use it as a set aswell
print("helloWorld","green")
print("helloWorld","blue")
print("helloWorld","yellow")
print("helloWorld","red")

// bellow you can see the print function arround a fileread
// print simply returns what you print and print it.
// you can use it to debug and remove it at anytime wihtout breaking code.
// just take care of the ()
filedata = print(fileread("./somefile.txt"),"red")  

//colors new::

colorarray = ["b","r","y","g","c","p","m"]

for color in colorarray{
    print(combine("the color =",color),color)
       // bright version br , bb ,bg etc
    print(combine("the color =",color),combine("b",color))
}



```
## /test.nscript :nscript core function
```swift
print("helloword!")
print("what is up ??")
iets = switch variable{
case 1 {
    print("oi")

}

```
## /fileexist :nscript core function
```swift
// if you like to check if a file exists you can use these 2 functions

check = fileexists("./checksome/file.txt")
if check == 1 {
    print("jep")
}

```
## /replace :nscript core function
```swift
// replace(strin,toreplace,replacewith) 
// this function replaces a substring in a string by another given string.


string = "Big John hates beer !"

string = replace(string,"beer","Microsoft") // as you can see the confusion arround Big John hating beer all got fixed. now Big John is back to himself again.

```
## /arrayshuffle :nscript core function
```swift
// this function allows you to instantly shuffle the entrees of a array.
// lets say you have a card game and the deck of cards is hold in a array.

array = ["beer","whiskey","is","it","friday","yet??"]

array = arrayshuffle(array) // <-- takes array shuffles it and set back variable array with the new aray data.

```
## /match :nscript core function
```swift
// following shows how to do a match (//switch) like thing

tocheck = "4"

match tocheck{
    "1" | "2" =>{
        print("first")
    }
    "3" => {
        print("second")
    }
    "4" => {
        print("fourth")
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}

// you can also catch a variable by a match
// on the match you can use a scope OR 1 word this be returned 
// to the variable. the last lines result is auto-returned.
myvar = match tocheck{
    "1" | "2" => print("first") //< no scope usage. print returns !
    "3" => "staticstring" //<-- static return
    "4" => {
        if @day == "01" {
            return "first day of the month" //<- use return to breakback
        }
        "not the first day"
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}

```
## /stringtohex :nscript core function
```swift
// these functions convert a string to a hex string , and back

hexstring = stringtohex("hello world !")
print(hextostring(hexstring)) 

```
## /weeks_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /arraysort :nscript core function
```swift
// this sorts a array to alphabetic/nummeric order

array = ["b","c","a"]
array = arraysort(array) //<-- returns a new array with sorted order

```
## /random :nscript core function
```swift
// this function returns a random number
// ranndom(minimum,maximim,roundedDecimals)

// returns a number 
number = random(1,100,0)

// returns a float with 1 decimal
number = random(1,100,1)

// no rounding at all
number = random(1,100)

```
## /split :nscript core function
```swift
// with this function you can split a string into a nscript arraystring and use it in loops

string = "Tommy|Karen|Herman"
array = split(string,"|")

for each in array {
    print(each)
}

```
## /instring :nscript core function
```swift
// you can use this to check if a substring is found in a string
// returns 0 if not found and 1 if found

string "hello world!"

if instring(string,"hello") == 1 {
    print("found it!")
}

```
## /function :nscript core function
```swift
// this example shows how to make a function wich takes arguments.
// ! the arguments names you assign inside the function scopes are local
// this ,means inside this scope those names only exist, as everything else in nscript is global by default.
// take this in mind! 

func myfirstfunction(name){
    ret = combine "hello " name " how are you?"
    return ret
} 

myreturnvar = myfirstfunction("Big John")
print(myreturnvar)

```
## /hextostring :nscript core function
```swift
// these functions convert a string to a hex string , and back

hexstring = stringtohex("hello world !")
print(hextostring(hexstring)) 

```
## /objtojson :nscript core function
```swift
// objtojson(obj) - objfromjson(objname,jsonstring)
// with these functions you can convert all properties of a obj/class
// to a json string and load it back in or sent it to other server to load it in,


class myclass{
    self.name = "superclass"
    self.descriptiuon = "im gonna be used for json"
}


// convert a class to json string
jsonstring = objtojson(myclass)

// load a new class name (arg 1) with a jsonstring (arg2) 
// and trow the objref back to the variable.
copiedclass = objfromjson("newuniqueclassreferencestring",jsonstring)

// you can now use the properties on this variable
// it will be performing on class newuniqueclassreferencestring
print(copiedclass.name,"green")

```
## /curl :nscript core function
```swift
// curl(url) this retrieves data from a url performing a get i believe.

get = curl("http://www.google.com")

```
## /filesize :nscript core function
```swift
// this function returns a fancy rounded format of the file size 
// related : filesizebytes(fname)

size = filesize("./testfile.bin") // lets say its 2000 bytes then this returns "2 KB" if its 20000000 it returns "2 MB"


```
## /round :nscript core function
```swift
// this function allows you to roundup a number by given decimals
// round(number,decimalstoround)


number = 0.499
rounded = round(number,1) // returns 0.5

```
## /else :nscript core function
```swift
// else is a scope wich you can use to trigger when a if scope was false, 
//unlike elseif this else has no condition else then the previous if scope been false.
// there can be lines between the 2 scopes they will be interpreted in between aswell. 


if 1 == 2 {
// not true!
}
else{
    print("true")
}

```
## /rawget :nscript core function
```swift
// raw get gets the response of a get to a unsecure http webhost


curlvar = rawget("192.168.1.66:8088","nscript_arrays.rs")

print(combine("curlvar:",curlvar),"blue")



```
## /exec :nscript core function
```swift
// this functions calls a new nscript file parsing, this will also load in class/func scopes.
// this can be used , include new script or to re-new class scopes during runtime by re-initializing them.

exec("./server.nc")

```
## /hours_in_ms :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

loop {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}

 

```
## /setobj :nscript core function
```swift
// set object is the function to spawn a object from a class.
// the function will automaticly trigger the construct function from the class tree, the last inherentance wich has this function scope initialized
// will clone the construct function to the spawning object, after all properties are cloned, the construct will run wich allows you to set using self.

class playerbase{
    func construct(){
        self.name = self
    }
}

setobj("playerbase","BigJohn")

```
## /inpool :nscript core function
```swift
// pool system, this is the same format as a array! but using pooladd() and poolremove() 
// will make sure that each entree in this array is unique !
// if an entree is pushed twice the array is unchanged
// pooladd(array,entree) , poolremove(array,entree) < - both return the array as result

array = ""
array = pooladd(array,"Joyce")
array = pooladd(array,"Kim")
array = pooladd(array,"Alice")
array = pooladd(array,"David") //<--- now this one is already in the pool and wont show

array = poolremove(array,"David") // <-- gues the ladies dont want David to be in the pool ! 

// inpool returns 1 on found and 0 if not found! --! Added on v2.005
if inpool(array,"Kim") == 1 {
	print("Seems Kim is in the pool.")
}

```
## /combine :nscript core function
```swift
// combine is a special line syntax and can also be used as a function
// it concetinates strings, or combines them


user = "Big John"
// the linebellow , all words of the line after combine , will be evaluated and concatinated
file = combine @scriptdir "database/" user ".db"
print(file)


ORrrrrrrrrrrrrrrrrr

// keep in mind a function cannot have spaces elsewhere of inside the "" this breaks the syntax if done!
// combine as a function call can take up to 9 arguments i believe
print(combine(@scriptdir,"database/",user,".db"))

```
## /load :nscript core function
```swift
// these are database like system to quickly store or load data to a file ( textformat )
// it saves the header on a line , and the line bellow it contains the data.
// headers must be unique per file, or it overwrites.

save("#header","data","filelocation")
data = load("#header","filelocation")

```
## /poolremove :nscript core function
```swift
// pool system, this is the same format as a array! but using pooladd() and poolremove() 
// will make sure that each entree in this array is unique !
// if an entree is pushed twice the array is unchanged
// pooladd(array,entree) , poolremove(array,entree) < - both return the array as result

array = ""
array = pooladd(array,"Joyce")
array = pooladd(array,"Kim")
array = pooladd(array,"Alice")
array = pooladd(array,"David") //<--- now this one is already in the pool and wont show

array = poolremove(array,"David") // <-- gues the ladies dont want David to be in the pool ! 

```
## /trimright :nscript core function
```swift
// this allows you to trim a string from the right side by a number of characters,
// it returns the new string

string = "BigJohn_users"
newstring = trimright(string,6) // will be BigJohn

```
## /arraypushroll :nscript core function
```swift
// this pushes a entree to a array as a roller system
// this means first-in-last-out
// entree 9 becomes 8 and entree 1 becomes 0 and entree 0 wont be in the array anymore.
// usage : website update panel, keep 10 items tops. using this will remain the size of the array.

array = ["im out first :( ","ill be next"]
rollerarray arraypushroll(array,"im in !")
rollerarray arraypushroll(array,"im in too !")

```
## /filecopy :nscript core function
```swift
// filecopy( file , destination) , copies a file and returns the status

status = filecopy("./beer.txt","./mybelly/beer.txt")

```
## /tcp :nscript core function
```swift
class tcp{
    func client(ip,port){
        
        tmr = timerinit()
        loop{
            self.serversocket = tcpconnect(ip,port)
            if self.serversocket != "" {
                print(cat("connected succesfully:",self.serversocket),"p")
                break
            }
            if timerdiff(tmr) > 999 {
                print("timed-out")
                tmr = timerinit()
            }
        }
        print(self.serversocket,"r")

    }
    func server(ip,port){
        self.listenersocked = tcplistener(ip,port)
        coroutine self.listenersocked{
            
            incsocket = tcpaccept(self)
            
            if incsocket != ""{

                print(cat("new socket connected:",incsocket),"p")
                coroutine incsocket{
                    res = tcpreceive(self)
                    if res != ""{
                        print(cat("socketloop:",self,"inc:[",res,"]"),"y")
                        tcpsend(self,"alrighty")
                    }                  
                }
            }
        }
        return self.listenersocked
    }
}
thread [c:tcp]{
    sleep(2)
    tcp.client("127.0.0.1",8888)
    timer = timerinit()
    coroutine "x"{
        if timerdiff(timer) > 999{
            tcpsend(tcp.serversocket,cat("t1=",@sec,",haii"))
            timer = timerinit()
            
        }

    }
}    
thread [c:tcp]{
    sleep(2)
    tcp.client("127.0.0.1",8888)
    timer = timerinit()
    coroutine "x"{
        if timerdiff(timer) > 679{
            tcpsend(tcp.serversocket,cat("t2=",@sec,",oiii"))
            timer = timerinit()
            
        }

    }
}   


tcp.server("127.0.0.1",8888)

```
## /run_program :nscript core function
```swift
// if you want to run another process you can use y
// if you need to escape the syntax and get a quote in your string use " quote=(\") "
// backslash quote - is how you can use a quote without breaking the interpreter

run("for --your --life --run --to --those --hills")

```
## /filemove :nscript core function
```swift
// filemove(file,destination) returns the status as a string

status = filemove("./windows11","./chemicalwastebin/neverrestoreme")

```
## /fileread :nscript core function
```swift
// reading a file is easy you can use
filedata = fileread("./file.txt")

```
## /base64tostring :nscript core function
```swift
// to encrypt or decrypt a file from binary to a string or from a string to binary 
// you can use a system called base64, ive implemented 2 functions
// base64tostring(base64satring) and stringtobase64(string)

print(base64tostring("QmlnIGpvaG4gbG92ZXMgY29sZCBiZWVycyE=")) // this will decrypt this top most important msg and print it.

//to pack a string
packed = stringtobase64("super secret stuff whooohooo!")

```
## /timerdiff :nscript core function
```swift
// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

while 1 {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}
if timerdiff(timer) > years_in_ms(2) {
}

 

```
## /zip :nscript core function
```swift
// to zip a directory to a zip file
//zip(directorytozip,zipfile)
// returns status || related unzip(file,dir)

status = zip("./testfolder/","./test.zip")

```
## /classes :nscript core function
```swift
//classes, this is where the real power of nscript comes to play, classes are unique referenced group object they can hold properties and functions
// a class can spawn new objects, this means a new reference is being made by a clone of a object, all properties and functions of the objects current state will be cloned.
// after a object clones all the properties it will execute a function called self.construct() you can use this to post-clone change things,



// when a class scope loads it will set a variable with the classname to a string holding the classname
// nscript classes have a string as a reference , a variable holding the class is nothing more then a string with the unique
// reference of the class as a "string"
class playerbase {
    func destruct(){
        print(combine("player:",self," deleted!")
    }
    func construct(){
        print(combine("player:",self," loaded!")
    }
// inside class scopes / functions / loops you can use the self.var self will be set to the object using the method.
// ((&self is a reflected variabe))
    self.usertype = "user"
}

class admin {
    self.usertype = "admin"
}

obj pete : playerbase
obj john : playerbase
obj john : admin

getallplayers = objgetchildren(playerbase) //<--- this function will get all spawned objects that popped out of the baseclass

getallprops = inobj(playerbase) // <--- this returns a array of all the propertie references of the oject

objdel(pete) // <-- this will execute the self.destruct() function if for example used in a game
// you be able to delete models and do some stuff etc. after the .destruct is done all properties and heritiges will be
//deleted.


// !! important , class variables function calls only work when the variable
// hold the same name as a string as the variable name
// if this is not the case you need to reflect
class something{
    func some(){
    }
//someclass
}

othername = objfromjson("actualname",objtojson(something))
// here variable othervar does not hold the same name , the reference of the object
// here is actualname, to reflect the classmethod use the *symbol ( used to be & but has been changed on v2.005)
*othername.some() // <-- triggerls actualname.some()
a = "some"
*othername.*a() // <-- triggerls actualname.some()

//reflecting also works on props and varnames

someobj = "someobj"
array = ["name","age","gender"]
array2 ["BigJohn","54","superalphamale"]
for x to 3 {
// the name of the property is here by reflected, and set with the data from the other array.
    someobj.*array[x] = array2[x]
}



```
## /dirlist :nscript core function
```swift
// dirtolist / dirlist / listdir(".dir")

array = listdir(@scriptdir)

for each in array {
    print(each)
}

```
## /fromleft :nscript core function
```swift
// with this function you get a new string from the left or right side of a stirng by a given number of characters
// fromright(string,charsasInt) fromleft()


string = "music.mp3"
filetype = fromright(string,4)

```
## /macros :nscript core function
```swift
//macros begin with a @ they are build in functions to return a value.


//special characters
@lf = a \n from a file linefeed.

//directory
@scriptdir

//Time
@year
@month
@day
@hour
@min
@sec
@msec

//system
@OS

//web-server ( these are set whenever some one triggers a .nc script on your webserver)
@socketip       // returns the IP adres of the connecting client.
//if using domains these be availible , are set when the user enters the http server based on his using domain
@webroot        // this refurs to ./domains/yourdomainname/
@webprivate     //this refurs to ./domains/yourdomainname/private/
@webpublic      //this refurs to ./domains/yourdomainname/public/

```
## /splitselect :nscript core function
```swift
// this function can be used to quickly fetch something from a string without getting a array first

email = "mambojambo@gmail.com"
name = splitselect(email,"@",0) // this will return the left side of the "@"

```
