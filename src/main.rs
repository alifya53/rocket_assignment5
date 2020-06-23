#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number{
    number : i32,
}

#[get("/num")]
fn get_request()-> Html<String>{
    let h1=format!("
    
    <div style='font-family: Helvetica, Arail; text-align: center;'>
    
        <h1 style='font-size: 50px;color:white; padding: 30px; background-color:#960612;'>Rust Rocket Server</h1>
        <img src='https://lh3.googleusercontent.com/4w3RSrDO8S1YUmsAQNJwaoZbp8Hmn3UQeVGQzyEDIZEWfIDZx-WSzysA4EOnz-X1MrpbTL8=s85' alt='rocketlogo'>
        <p style='font-size: 40px;color:#410208; font-weight: bolder;'>Please Enter a Number.</p>
        <form style='margin: 100px 0px; margin-bottom: 80px;' action='/num' method='post'>
            <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
            width: 500px; font-size: 24px; border: 1px solid #960612;' type='number' placeholder='Enter number'
                name='number' id='number'> </br>
                </br>
            <input style='font-size: 24px; padding:5px 12px; 
            border: none; height: 40px;
            border-radius: 3px;  color: white; background-color: #960612
            ;' type='Submit'>
        </form>
        
    </div>
    ");
    Html(h1)
}

#[post("/num", data = "<number>")]
fn recieve_request(number : Form<Number>) -> Html<String> {
   let h1=format!("
   <div style='font-family: Helvetica, Arail; text-align: center;'>
   <h1 style='font-size: 50px;color:white; padding: 30px 20px; background-color:#960612;border-radius: 5px'>Rust Rocket Server</h1>
   <img src='https://lh3.googleusercontent.com/4w3RSrDO8S1YUmsAQNJwaoZbp8Hmn3UQeVGQzyEDIZEWfIDZx-WSzysA4EOnz-X1MrpbTL8=s85' alt='rocketlogo'>  
   <p style='font-size: 35px; color:#410208; font-weight: bolder;'>
   Given Number is => {} </br> and Result is =>
   {}</p>
   
   </div>
    <div 'style='position: absolute;'><a href='https://www.github.com/alifya53' style='padding: 10px 20px; margin-top: 10px; 
            font-size: 24px; color: white; background-color: #960612; border-radius: 5px;text-aligh: center;'>Github</a></div>
    ", number.number, number.number+10);
    Html(h1)
}


#[get("/")]
fn home()-> Html<String>{
    let html=format!("
    <div
        style='position: relative; font-family: Helvetica, Arail; text-align: center; width: 100%; height:100vh; overflow: hidden;'>
        
        <div
            style='position: absolute; top:0; background-color: #b63640; width: inherit; height: inherit; opacity: 0.3;'>
          </div>
          <div style='position: absolute; top:40%; left: 40%;'>
          <img src='https://lh3.googleusercontent.com/4w3RSrDO8S1YUmsAQNJwaoZbp8Hmn3UQeVGQzyEDIZEWfIDZx-WSzysA4EOnz-X1MrpbTL8=s85' alt='rocketlogo'>
            <h1
                style='margin-bottom:  10px; font-size: 4rem; font-weight: bolder; color: white;  text-shadow: 2px 2px 4px #000000;'>
                Rocket Server</h1>
                <p style='font-size: 35px; color:#410208; font-weight: bolder;'> Assignment 5 </p>
            <a href='/num' style='padding: 10px 20px; margin-top: 10px; 
            font-size: 24px; color: white; background-color: #960612; text-decoration: none; border-radius: 5px;'>Login</a>
        
        </div>");
    Html(html)
}


fn main() {
    rocket::ignite().mount("/", routes![home, get_request, recieve_request]).launch();
}

