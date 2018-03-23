#[macro_use]
extern crate json_api;

struct Person {
    id: u64,
    name: String,
    symbol: String,
}

resource!(Person, |&self| {
    kind "heroes";
    id self.id;

    attrs name, symbol;
});

#[test]
fn doc_from_str() {
    let json = r#"{
      "data": [{
        "type": "heroes",
        "id": "1",
        "attributes": {
          "name": "Batman",
          "symbol": "🦇"
        }
      }, {
        "type": "heroes",
        "id": "2",
        "attributes": {
          "name": "Wonder Woman",
          "symbol": "🛡"
        }
      }]
    }"#;

    let doc = json_api::from_str(json);

    assert!(false);
}
