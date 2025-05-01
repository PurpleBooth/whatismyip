use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use futures::executor::block_on;
use miette::Result;
use std::net::{IpAddr, Ipv4Addr};
use whatismyip::myip::MyIp;
use whatismyip::{IpVersion, MyIps, find_local_ip, process_ips};

fn bench_process_ips(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_ips");

    // Create test data with different numbers of IPs
    for size in [1, 10, 50, 100].iter() {
        let ips: MyIps = (0..*size)
            .map(|i| {
                let ip = Ipv4Addr::new(192, 168, 1, i as u8);
                MyIp::new_plain(IpAddr::V4(ip))
            })
            .collect();

        let results: Vec<Result<MyIps>> = vec![Ok(ips)];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| block_on(process_ips(&results, false)));
        });
    }

    group.finish();
}

fn bench_find_local_ip(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_local_ip");

    group.bench_function("ipv4", |b| b.iter(|| find_local_ip(Some(IpVersion::Ipv4))));

    group.bench_function("ipv6", |b| b.iter(|| find_local_ip(Some(IpVersion::Ipv6))));

    group.bench_function("both", |b| b.iter(|| find_local_ip(None)));

    group.finish();
}

criterion_group!(benches, bench_process_ips, bench_find_local_ip);
criterion_main!(benches);
