#[cfg(test)]
mod unit {
    use anyhow::Result;
    // use crate::s3::list_buckets;

    #[tokio::test]
    async fn test_one() -> Result<()> {
        // let buckets = list_buckets().await?;
        // println!("{:?}", buckets);
        Ok(())
    }

    #[tokio::test]
    async fn test_two() -> Result<()> {
        // let buckets = list_buckets().await?;
        // println!("{:?}", buckets);
        Ok(())
    }

    mod polly {
        use anyhow::Result;
        use tracing_subscriber::FmtSubscriber;

        #[tokio::test]
        async fn test_two() -> Result<()> {
            let subscriber = FmtSubscriber::builder()
                .with_max_level(tracing::Level::DEBUG)
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
            Ok(())
        }
    }
}
