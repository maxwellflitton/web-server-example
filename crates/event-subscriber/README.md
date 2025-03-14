# Nanoservices event subscriber

A basic proc macro crate for subscribing to events. We can subscribe to events with the following code:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Two;

#[subscribe_to_event]
async fn test2(two: Two) {
    println!("calling from test2 function with: {:?}", two);
}
```

This generates the code for a standard function under the name as it is defined as, and a slightly mangled function which is registered with the event subscriber. If an event is then published with the `Two` then all functions subscribed to the `Two` event will be called with the instance of the `Two` struct. 
