use assignment7_lib;
mod module1;
mod pakistan {
	pub mod islamabad {
		pub fn high_city() {
			println!("we are in high_city function");
			}
		pub fn first_test() {
			println!("we are in first_test function");
			}
	
		pub mod  smart {
			pub fn rawalpindi() {
			println!("we are in rawalpindi function");
				}
	}		}
}
use pakistan::islamabad;
use pakistan::islamabad::smart;

fn main() {
  	println!("####Assignment7 Q# 1####");
	println!("");

	pakistan::islamabad::high_city();
	crate::pakistan::islamabad::high_city();
	islamabad::high_city();
	println!("");
    	println!("calling from islamabad mod");
	println!("");
	pakistan::islamabad::first_test();
	crate::pakistan::islamabad::first_test();
	islamabad::first_test();
	println!("");
    	println!("calling from rawalpindi mod");
	println!("");
	pakistan::islamabad::smart::rawalpindi();
	crate::pakistan::islamabad::smart::rawalpindi();
	smart::rawalpindi();
	println!("Q1 ended");
	println!("");
    	println!("Assignment_7 Q# 2");
	println!("");
	println!("calling from module.rs");
	module1::lawyer_colony();
	module1::chaklala();
	println!("");
	module1::punjab::house::room();
	module1::punjab::city::murree();
	println!("Q# 2 ended");
	println!("");
    	println!("Assignment_7 Q# 3");
	println!("");
	assignment7_lib::tests::it_works();
	assignment7_lib::rust::it_compile();
	println!("Q# 3 ended");

}
