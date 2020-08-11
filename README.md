# Presign

> Generate S3 presigned URLs for both put and get

![Rust](https://github.com/hoop33/presign/workflows/Rust/badge.svg)

## Overview

You can generate presigned URLs for S3 objects using the AWS CLI, but for GET only (from what I can tell). This is a simple CLI for generating presigned URLs for either GET or PUT.

## Usage

```sh
presign 0.1.0
Generate a presigned S3 URL for GET or PUT

USAGE:
    presign [OPTIONS] --bucket-name <bucket-name> --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bucket-name <bucket-name>    The bucket name for the presigned URL
    -e, --expiration <expiration>      The expiration, in seconds, for the presigned URL [default: 3600]
    -f, --file <file>                  The file for the presigned URL
    -m, --method <method>              The method for the presigned URL [default: put]
    -p, --profile <profile>            The AWS profile to use
    -r, --region <region>              The AWS region the bucket is in [default: us-east-1]
```

For example, to create a presigned URL to PUT file `test.txt` in bucket `mytest`:

```sh
$ presign --bucket-name mytest --file test.txt
```

To create a presigned URL to GET file `test.txt` in bucket `mytest` with a 5 minute expiration:

```sh
$ presign --bucket-name mytest --file test.txt --expiration 300
```

## Credits

Thanks to the developers / maintainers of these libraries:

* [structopt](https://github.com/TeXitoi/structopt)
* [rust-s3](https://github.com/durch/rust-s3)

If you look at the source, you'll see it's _very thin_ wrapper of those two crates.

## License

Copyright &copy; 2020 Rob Warner

Licensed under the [MIT License](https://hoop33.mit-license.org/)

