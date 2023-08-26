use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

fn main() {
    let resolver =
        Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();

    let response = resolver
        .lookup_ip("example.com")
        .unwrap()
        .iter()
        .next()
        .unwrap();

    println!("{}", response);
}
