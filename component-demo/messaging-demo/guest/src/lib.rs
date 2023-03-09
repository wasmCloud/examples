use crate::{
    messaging_types::{open_broker, ChannelParam, EventParam},
    producer::publish,
};

wit_bindgen::generate!({
    path: "../wit",
    world: "messaging",
});

struct MyMessaging;

impl handler::Handler for MyMessaging {
    fn on_receive(e: messaging_types::EventResult) -> Result<(), u32> {
        println!(">>> Received: {:#?}", e);

        let data = e.data.unwrap();
        let data_s = String::from_utf8(data).unwrap();

        // process the data
        let msg = fizz_buzz(data_s.as_str());

        // published the processed data
        let b = open_broker("my-messaging")?;
        let new_event = EventParam {
            data: Some(msg.as_bytes()),
            id: "123",
            source: "rust",
            specversion: "1.0",
            ty: "com.my-messaing.rust.fizzbuzz",
            datacontenttype: None,
            dataschema: None,
            subject: None,
            time: None,
            extensions: None,
        };

        println!(">>> Publishing: {:#?}", new_event);
        publish(b, ChannelParam::Topic("rust"), new_event)?;

        Ok(())
    }
}

export_messaging!(MyMessaging);

// replace all instances of fizz in a word w/ buzz
fn fizz_buzz(word: &str) -> String {
    word.replace("fizz", "buzz")
}
