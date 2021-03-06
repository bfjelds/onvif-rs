extern crate onvif_rs;
use onvif_rs::{schema, soap};

struct Opts {
    uri: String,
    username: String,
    password: String,
    hostname: String,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let mut client = soap::client::Client::new(
        &opts.uri,
        Some(soap::client::Credentials {
            username: opts.username,
            password: opts.password,
        }),
    );

    let response = schema::devicemgmt::set_hostname(
        &mut client,
        &schema::devicemgmt::SetHostname {
            name: opts.hostname,
        },
    )
    .await;

    println!("set_hostname(): {:#?}", response);

    if response.is_ok() {
        println!(
            "get_hostname(): {:#?}",
            schema::devicemgmt::get_hostname(&mut client, &Default::default()).await
        );
    }
}

impl Opts {
    pub fn parse() -> Opts {
        let opts: Result<Opts, ()> = (|| {
            let mut args = std::env::args().skip(1);

            Ok(Opts {
                uri: args.next().ok_or(())?,
                username: args.next().ok_or(())?,
                password: args.next().ok_or(())?,
                hostname: args.next().ok_or(())?,
            })
        })();

        match opts {
            Ok(opts) => opts,
            Err(_) => {
                eprintln!(
                    "Usage: {} URI USERNAME PASSWORD HOSTNAME",
                    std::env::args()
                        .next()
                        .unwrap_or("camera_set_hostname".into())
                );
                std::process::exit(1);
            }
        }
    }
}
