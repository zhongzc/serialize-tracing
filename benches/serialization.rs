use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, Copy, Clone)]
enum SerType {
    Flatbuffers,
    Protobuf,
}

#[derive(Debug, Copy, Clone)]
enum EventType {
    Str,
    Num,
}

fn serialization(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "serialization",
        |b, (ser_t, event_t, c)| {
            b.iter_with_setup(
                || serialize_tracing::gen_span::rand_spans(**c),
                |spans| match (ser_t, event_t) {
                    (SerType::Flatbuffers, EventType::Num) => {
                        black_box(serialize_tracing::serialize_fbs_num_event(&spans))
                    }
                    (SerType::Flatbuffers, EventType::Str) => {
                        black_box(serialize_tracing::serialize_fbs_str_event(&spans))
                    }
                    (SerType::Protobuf, EventType::Num) => {
                        black_box(serialize_tracing::serialize_pb_num_event(&spans))
                    }
                    (SerType::Protobuf, EventType::Str) => {
                        black_box(serialize_tracing::serialize_pb_str_event(&spans))
                    }
                },
            );
        },
        {
            let mut inputs = vec![];

            for s in &[SerType::Flatbuffers, SerType::Protobuf] {
                for e in &[EventType::Num, EventType::Str] {
                    for c in &[100, 1000] {
                        inputs.push((s, e, c));
                    }
                }
            }

            inputs
        },
    );
}

criterion_group!(benches, serialization);
criterion_main!(benches);
