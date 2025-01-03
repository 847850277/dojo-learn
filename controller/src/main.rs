mod controller_test;

#[tokio::main]
async fn main() {
    controller_test::deploy_controller_test::test_deploy_controller().await;

}