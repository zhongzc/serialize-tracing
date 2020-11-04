#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
#[allow(non_snake_case)]
#[allow(unused_imports)]

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use protos::tracing as tracing_pb;

pub mod tracing_fbs {
    include!(concat!(
        env!("OUT_DIR"),
        "/flatbuffers/tracing_generated.rs"
    ));
}

pub mod gen_span;
use crate::gen_span::Span;
use protobuf::Message;

pub fn serialize_pb_str_event(spans: &[Span]) -> Vec<u8> {
    let mut spans_vec = Vec::with_capacity(spans.len());

    for s in spans {
        let mut properties = Vec::with_capacity(s.properties.len());
        for property in &s.properties {
            properties.push(tracing_pb::Property {
                key: property.key.clone(),
                value: property.value.clone(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            });
        }
        spans_vec.push(tracing_pb::Span1 {
            start_time_ns: s.start_time_ns,
            duration_ns: s.duration_ns,
            id: s.id,
            parent_id: s.parent_id,
            event: s.event_str.clone(),
            properties: properties.into(),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        });
    }

    let spans = tracing_pb::Spans1 {
        spans: spans_vec.into(),
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    };

    spans.write_to_bytes().unwrap()
}

pub fn serialize_fbs_str_event(spans: &[Span]) -> Vec<u8> {
    let mut fbb = flatbuffers::FlatBufferBuilder::new_with_capacity(4096);

    let mut fbs_spans = Vec::with_capacity(spans.len());
    for s in spans {
        let mut properties = Vec::with_capacity(s.properties.len());
        for p in &s.properties {
            let key = fbb.create_vector_direct(p.key.as_bytes());
            let value = fbb.create_vector_direct(p.value.as_bytes());
            properties.push(tracing_fbs::Property::create(
                &mut fbb,
                &tracing_fbs::PropertyArgs {
                    key: Some(key),
                    value: Some(value),
                },
            ))
        }

        let event = fbb.create_vector_direct(s.event_str.as_bytes());
        let properties = fbb.create_vector(&properties);
        fbs_spans.push(tracing_fbs::Span1::create(
            &mut fbb,
            &tracing_fbs::Span1Args {
                start_time_ns: s.start_time_ns,
                duration_ns: s.duration_ns,
                id: s.id,
                parent_id: s.parent_id,
                event: Some(event),
                properties: Some(properties),
            },
        ));
    }

    let spans_vec = fbb.create_vector(&fbs_spans);
    let spans = tracing_fbs::Spans1::create(
        &mut fbb,
        &tracing_fbs::Spans1Args {
            spans: Some(spans_vec),
        },
    );

    fbb.finish(spans, None);
    Vec::from(fbb.finished_data())
}

pub fn serialize_pb_num_event(spans: &[Span]) -> Vec<u8> {
    let mut spans_vec = Vec::with_capacity(spans.len());

    for s in spans {
        let mut properties = Vec::with_capacity(s.properties.len());
        for property in &s.properties {
            properties.push(tracing_pb::Property {
                key: property.key.clone(),
                value: property.value.clone(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            });
        }
        spans_vec.push(tracing_pb::Span2 {
            start_time_ns: s.start_time_ns,
            duration_ns: s.duration_ns,
            id: s.id,
            parent_id: s.parent_id,
            event: s.event_num,
            properties: properties.into(),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        });
    }

    let spans = tracing_pb::Spans2 {
        spans: spans_vec.into(),
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    };

    spans.write_to_bytes().unwrap()
}

pub fn serialize_fbs_num_event(spans: &[Span]) -> Vec<u8> {
    let mut fbb = flatbuffers::FlatBufferBuilder::new_with_capacity(4096);

    let mut fbs_spans = Vec::with_capacity(spans.len());
    for s in spans {
        let mut properties = Vec::with_capacity(s.properties.len());
        for p in &s.properties {
            let key = fbb.create_vector_direct(p.key.as_bytes());
            let value = fbb.create_vector_direct(p.value.as_bytes());
            properties.push(tracing_fbs::Property::create(
                &mut fbb,
                &tracing_fbs::PropertyArgs {
                    key: Some(key),
                    value: Some(value),
                },
            ))
        }

        let properties = fbb.create_vector(&properties);
        fbs_spans.push(tracing_fbs::Span2::create(
            &mut fbb,
            &tracing_fbs::Span2Args {
                start_time_ns: s.start_time_ns,
                duration_ns: s.duration_ns,
                id: s.id,
                parent_id: s.parent_id,
                event: s.event_num,
                properties: Some(properties),
            },
        ));
    }

    let spans_vec = fbb.create_vector(&fbs_spans);
    let spans = tracing_fbs::Spans2::create(
        &mut fbb,
        &tracing_fbs::Spans2Args {
            spans: Some(spans_vec),
        },
    );

    fbb.finish(spans, None);
    Vec::from(fbb.finished_data())
}
