//! Defines the macro around mapping functions to traits for transactions.

#[macro_export]
macro_rules! define_dal_transactions {
    (
        $( $trait:ident => $func_name:ident $(< $($generic:tt),* >)? ($($param:ident : $ptype:ty),*) -> $rtype:ty ),* $(,)?
    ) => {
        $(
            pub trait $trait {
                fn $func_name $(< $($generic),* >)? ($($param : $ptype),*) -> impl std::future::Future<Output = Result<$rtype, utils::errors::NanoServiceError>> + Send;
            }
        )*
    };
}


#[cfg(test)]
mod tests {

    use dal_tx_impl::impl_transaction;
    use utils::errors::NanoServiceError;
    use std::future::Future;

    struct TestStruct;

    trait TestTrait {
        fn test_fn() -> impl Future<Output = Result<i32, NanoServiceError>> + Send;
    }

    #[impl_transaction(TestStruct, TestTrait, test_fn)]
    async fn test_fn() -> Result<i32, NanoServiceError> {
        Ok(35)
    }

    #[tokio::test]
    async fn test_impl_transaction() {
        let outcome = TestStruct::test_fn().await;
        assert_eq!(outcome.unwrap(), 35);
    }

    #[tokio::test]
    async fn test_define_dal_transactions() {

        struct NewUser;

        define_dal_transactions!(
            CreateUser => create(user: NewUser) -> i32,
            DeleteUser => delete(id: i32) -> bool
        );

        struct PostgresHandle;

        #[impl_transaction(PostgresHandle, DeleteUser, delete)]
        async fn create_user_postgres(_uid: i32) -> Result<bool, NanoServiceError> {
            Ok(true)
        }

        #[impl_transaction(PostgresHandle, CreateUser, create)]
        async fn create_user_postgres(_user: NewUser) -> Result<i32, NanoServiceError> {
            Ok(1)
        }
        let new_user = NewUser;
        let outcome = PostgresHandle::create(new_user).await.unwrap();
        assert_eq!(outcome, 1);

        let outcome = PostgresHandle::delete(1).await.unwrap();
        assert_eq!(outcome, true);

    }

}
