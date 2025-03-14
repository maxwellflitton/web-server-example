# Dal Tx Impl

A basic proc macro crate for implementing async functions into traits. Below, we can define the following trait:

```rust
trait TestTrait {
    fn test_fn() -> impl Future<Output = Result<i32, NanoServiceError>> + Send;
}
```

We can then implement the `TestTrait` trait for the `TestStruct` using the `impl_transaction` macro with the code below:

```rust 
#[impl_transaction(TestStruct, TestTrait, test_fn)]
async fn any_function_name() -> Result<i32, NanoServiceError> {
    Ok(35)
}
```

This macro is for single async functions only to use against traits with just one function to implement. The name of the function is just for readability as the body is lifted into the trait function implementation so there are no clashes with other functions. This macro can also be helpful with mocking.
