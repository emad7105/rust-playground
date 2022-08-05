mod async_select_join_all;
mod async_select_ok_with_error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //async_select_join_all::run().await?;
    async_select_ok_with_error::run().await?;

    Ok(())
}