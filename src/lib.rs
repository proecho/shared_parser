use std::boxed::Box;
use objects::*;
use std::str::FromStr;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,NaiveTime};


//puts informtion in each objects and internal feild and creates objects
//takes vectors of string which consist of feild title followed by elements of single words split from next title by newlinw
// eg ["Title", "timmy", "is", "cool", "/n", Datetime, "a_date_time", "/n", "List", "tomato", "carrot", "leek", "/n"] 
pub fn Event_parser< T: entry_type>(input:Vec<&str>,heading_list:Vec<String>) -> T {
	let mut output:Vec<Option<String>> = Vec::new();
	
	for heading in heading_list{
	    let mut interim_output: Vec<String> = Vec::new();
		let mut catch = false;
		for value in input.clone() {
			if value == "\n"{
				catch=false;
			}
			if catch == true{
			    interim_output.push(value.to_string());
			}
		    if value == heading{
				catch = true;				
			}
		}
		if interim_output == (Vec::<String>::new()){
			output.push(None)
		}else{
		    output.push(Some(unifier(interim_output)));
	    }
			
	    
	}
    let Title = output[0].clone();
    let DateTime= match output[1].clone(){
		Some(a) => Box::new(Some(DateTime::<Utc>::from_str(&a).unwrap())),
		None => Box::new(None),
	};
	let List = output[2].clone();
    let Other = output[3].clone();
			
	
	return T::new(
	    Title,
	    DateTime,
	    List,
	    Other,
	);
}

//converts vectors of strings into single string
fn unifier(vector:Vec<String>)->String{
	let mut output = String::new(); 
	for a in vector {
		output = format!("{} {}", output, a);
	}
	
	output
}
//only use for values that you know definately have datetime components will panic if none
pub fn time_inside(value:&entrys) -> NaiveTime {
	match value {
	    entrys::Todo(a) => return a.get_date_time().unwrap().time(),
		entrys::Events(b) =>return b.get_date_time().unwrap().time(),
		entrys::appointments(c) =>return c.get_date_time().unwrap().time(),
	}
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn test_unifier(){
		let test_vector = vec!["a","cute","cat"];
		let test_string = unifier(test_vector);
		assert_eq!(test_string,"a cute cat".to_string());
	}
	fn test_time_inside(){
		let original_date_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
		let title = "timmy".to_string();
		let list ="buy tomato".to_string();
		let datetime = Box::new(Some(original_date_time.clone()));
		let other = "sdnjabjlkvkjbaj".to_string();
		
		let tester = Todo::new(Some(title), datetime, Some(list), Some(other));
		let time = tester.time_inside();
		assert_eq!(time,original_date_time.time());
	}
	
}
