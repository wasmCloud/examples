wit_bindgen::generate!("kvdemo" in "../wit");

// /// Main struct that will implement wasi component imports/exports
struct Demo;

const BUCKET: &str = "bucket";
const KEY: &str = "hello";
const VALUE: &str = "wasi";

fn run() -> Result<Option<Vec<u8>>, String> {
    // Check if the value exists already
    println!("[info][kvdemo] checking for key [{KEY}]");
    let exists = keyvalue::exists(KEY).map_err(|_| format!("ERROR: failed to check for key [{KEY}]"))?;

    // If the value exists, print it
    if exists {
        println!("[info][kvdemo] value exists for key [{KEY}], reading...");
        let value = keyvalue::get(KEY).map_err(|_| format!("ERROR: get for [{KEY}] failed"))?;
        println!("existing value for key [{KEY}] is [{VALUE}]");

        // Delete the key so the next run sets
        let _ = keyvalue::delete(KEY).map_err(|_| format!("ERROR: failed to delete existing value"))?;
        println!("[info][kvdemo] successfully deleted value for key [{KEY}]");

        return Ok(None);
    }

    // Set the value to a given key
    println!("[info][kvdemo] setting key [{KEY}] to value [{VALUE}]");
    let _ = keyvalue::set(KEY, VALUE).map_err(|_| format!("ERROR: failed to set [{KEY}] to [{VALUE}]"))?;

    // Retrieve the value that was just set
    println!("[info][kvdemo] re-retrieving value for key: [{KEY}]");
    let value = keyvalue::get(KEY).map_err(|_| format!("ERROR: failed to retrieve value after set"))?;

    Ok(None)
}

impl actor::Actor for Demo {
    fn guest_call(op: String, _payload: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
        match op.as_str() {
            "Actor.Run" => run(),
            _ => Err(format!("Operation [{op}] not supported")),
        }
    }
}

export_kvdemo!(Demo);
