extern crate rustorm;
extern crate codegenta;

use codegenta::generator;
use codegenta::generator::Config;
use rustorm::pool::ManagedPool;
use codegenta::table_json::TableJson;

/// this will generate needed model code for tests in ./examples/gen directory
fn main(){
    
    let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v8", 1).unwrap();
    let db = pool.connect().unwrap();
    let tables = generator::get_all_tables(db.as_dev());
	for table in &tables{
		if table.name == "product" {
			let json = table.to_json();
			println!("json: {}", json);
		}
	}
}

