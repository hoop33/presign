use s3::{bucket::Bucket, creds::Credentials};
use std::{boxed::Box, error::Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "presign", about = "Generate a presigned S3 URL for GET or PUT")]
struct Cli {
    #[structopt(
        short,
        long,
        default_value = "us-east-1",
        help("The AWS region the bucket is in")
    )]
    region: String,

    #[structopt(short, long, help("The bucket name for the presigned URL"))]
    bucket_name: String,

    #[structopt(short, long, help("The file for the presigned URL"))]
    file: String,

    #[structopt(
        short,
        long,
        default_value = "put",
        help("The method for the presigned URL")
    )]
    method: String,

    #[structopt(
        short,
        long,
        default_value = "3600",
        help("The expiration, in seconds, for the presigned URL")
    )]
    expiration: u32,

    #[structopt(short, long, help("The AWS profile to use"))]
    profile: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let credentials = Credentials::from_profile(args.profile.as_deref())?;
    let bucket = Bucket::new(&args.bucket_name, args.region.parse()?, credentials)?;
    let url = match args.method.as_str() {
        "put" => bucket.presign_put(&args.file, args.expiration),
        _ => bucket.presign_get(&args.file, args.expiration),
    };
    match url {
        Ok(url) => {
            println!("{}", url);
            return Ok(());
        }
        Err(err) => return Err(Box::new(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_options() -> Result<(), String> {
        let vec = vec![
            "presign",
            "--bucket-name",
            "mybucket",
            "--expiration",
            "60",
            "--file",
            "foo.txt",
            "--method",
            "get",
            "--profile",
            "awsprofile",
            "--region",
            "us-west-2",
        ];
        let args = Cli::from_iter(vec.iter());
        assert_eq!(args.bucket_name, "mybucket");
        assert_eq!(args.expiration, 60);
        assert_eq!(args.file, "foo.txt");
        assert_eq!(args.method, "get");
        assert_eq!(args.profile, Some("awsprofile".to_string()));
        assert_eq!(args.region, "us-west-2");
        Ok(())
    }

    #[test]
    fn test_defaults() -> Result<(), String> {
        let vec = vec!["presign", "--bucket-name", "mybucket", "--file", "foo.txt"];
        let args = Cli::from_iter(vec.iter());
        assert_eq!(args.bucket_name, "mybucket");
        assert_eq!(args.expiration, 3600);
        assert_eq!(args.file, "foo.txt");
        assert_eq!(args.method, "put");
        assert_eq!(args.profile, None);
        assert_eq!(args.region, "us-east-1");
        Ok(())
    }
}
