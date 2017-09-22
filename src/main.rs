extern crate iron;
extern crate router; 

use iron::prelude::*;
use iron::status;
use router::Router;
 
fn main() {
  let mut router = Router::new();
  router.get("/", hello_world, "Index");
  
  fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!asd asd asdas dasd assssssssssssssssssddsfsdf sd sd sd sds hsdjk hkds klsdsfklsdkjfsdkfskjdhf ksjdfjksdhkjsdhkjs
    	ssd f
    	sd 
    	ds f
    	ds

    	 sd
    	  ds

    	  dsf ds
    	  sd
    	  sd
    	  s
    	  sd
    	  s 
    	  sd
    	  ds
    	      	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	      	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	       	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	  
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	   szzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
    	    dlsds; sls da sd sadas ddas sas ada sas sa sa sas dsa dsa das as")))
  }
 
  Iron::new(router).http("localhost:4000").unwrap();
  println!("On 4000");
}
