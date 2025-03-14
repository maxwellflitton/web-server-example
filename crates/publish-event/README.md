# Nanoservices event publisher

A basic proc macro crate for publishing events. We can subscribe to events with the following code:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Two;

#[subscribe_to_event]
async fn test2(two: Two) {
    println!("calling from test2 function with: {:?}", two);
}
```

This generates the code for a standard function under the name as it is defined as, and a slightly mangled function which is registered with the event subscriber. If an event is then published with the `Two` then all functions subscribed to the `Two` event will be called with the instance of the `Two` struct. We can then publish the event with the following code:

```rust
let two = Two {};
publish_event!(two);
```

The `test2` function will then be called with the `Two` struct instance. This is a basic event publisher and subscriber system for nanoservices. 
