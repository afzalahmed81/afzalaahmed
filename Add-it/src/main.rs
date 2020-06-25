// -----------------------Rust Server with Rocket Framework-----------------
//------------------------Assignment 05-------------------------------------
// Afzal Ahmed, Batch 2, IOT046458-------------------------------------------

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Addnumb{
    numb : i32,
}

#[get("/numb")]
fn request_server()-> Html<String>{
    let h1=format!("
    <div 
        style='text-align: center; padding: 5px '>      
        <h1> Rust Server with Rocket Framework </h1> </br> 
        <p style='font-size: 20px;'>Enter any number, Sever will respond by adding 10 to the given number.</p>
        <form style='margin: 100px 0px; margin-bottom: 80px;' action='/numb' method='post'>
            <input style='padding: 5px 10px; border-radius: 5px;  height: 50px;
            width: 250px; font-size: 24px; border: 1px solid #9e00c5;' type='numb' placeholder='Enter number'
                name='numb' id='numb'>
            <input style='font-size: 24px; padding:5px 12px; 
            border: none; height: 40px;
            border-radius: 5px;  color: maroon; background-color: cyan

            ;' type='Submit'>
        </form>
        
    </div>
    ");
    Html(h1)
}
        
#[post("/numb", data = "<numb>")]
fn add_update(numb : Form<Addnumb>) -> Html<String> {
   let h1=format!("
   <div 
   style='text-align: center; padding: 5px '>      
        <h1> Rust Server with Rocket Framework </h1> </br> 
   <p style='font-size: 24px; color:green; font-weight: bolder;'> </br> </br>
   The entered number is {} and the result is: {} </p>
   
   </div>
    ", numb.numb, numb.numb+10);
    Html(h1)
}


#[get("/")]
fn start()-> Html<String>{
    let html=format!("
        <div   
            style='text-align: center; padding: 5px '>      
            <h1> Rust Server with Rocket Framework </h1> </br> </br> </br>

            <a href='/numb' style= 'font-size: 36px'> Start Server </a>
        </div>");
    Html(html)
}


fn main() {
    rocket::ignite().mount("/", routes![start, request_server,add_update]).launch();
}

