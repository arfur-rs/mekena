use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mekena::prelude::*;

pub async fn send_and_recv_1_000_000<M: Message + Copy + 'static>(mailbox: &Mailbox, message: M) {
    for _ in 1..1_000_000 {
        mailbox.send(message).await.unwrap();
        mailbox.recv::<M>().await.unwrap();
    }
}

pub fn messaging(c: &mut Criterion) {
    let mailbox = Mailbox::new();

    c.bench_with_input(
        BenchmarkId::new("stress_test_1_000_000", i32::MAX),
        &i32::MAX,
        |b, &s| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| send_and_recv_1_000_000(&mailbox, s));
        },
    );
}

criterion_group!(benches, messaging);
criterion_main!(benches);
