mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use protos::tracing as tracing_pb;

#[path = "../target/flatbuffers/tracing_generated.rs"]
pub mod tracing_fbs;

pub mod gen_span;
use crate::gen_span::Span;
use protobuf::Message;

pub fn serialize_pb_str_event(spans: &[Span]) -> Vec<u8> {
    let mut res_spans = tracing_pb::Spans1::default();
    res_spans.set_spans(protobuf::RepeatedField::from_vec(Vec::with_capacity(
        spans.len(),
    )));
    let mut_spans = res_spans.mut_spans();

    for s in spans {
        let span = mut_spans.push_default();
        for p in &s.properties {
            let property = span.mut_properties().push_default();
            property.set_key(p.key.clone());
            property.set_value(p.value.clone());
        }
        span.set_start_time_ns(s.start_time_ns);
        span.set_duration_ns(s.duration_ns);
        span.set_id(s.id);
        span.set_parent_id(s.parent_id);
        span.set_event(s.event_str.clone());
    }

    res_spans.write_to_bytes().unwrap()
}

pub fn serialize_fbs_str_event(spans: &[Span]) -> Vec<u8> {
    let mut fbb = flatbuffers::FlatBufferBuilder::new_with_capacity(4096);

    let mut fbs_spans = Vec::with_capacity(spans.len());
    let mut properties_vec = Vec::with_capacity(64);

    for s in spans {
        for p in &s.properties {
            let key = fbb.create_vector_direct(p.key.as_bytes());
            let value = fbb.create_vector_direct(p.value.as_bytes());
            properties_vec.push(tracing_fbs::Property::create(
                &mut fbb,
                &tracing_fbs::PropertyArgs {
                    key: Some(key),
                    value: Some(value),
                },
            ))
        }

        let event = fbb.create_vector_direct(s.event_str.as_bytes());
        let properties = fbb.create_vector(&properties_vec);
        properties_vec.clear();

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
    let mut res_spans = tracing_pb::Spans2::default();
    res_spans.set_spans(protobuf::RepeatedField::from_vec(Vec::with_capacity(
        spans.len(),
    )));
    let mut_spans = res_spans.mut_spans();

    for s in spans {
        let span = mut_spans.push_default();
        for p in &s.properties {
            let property = span.mut_properties().push_default();
            property.set_key(p.key.clone());
            property.set_value(p.value.clone());
        }
        span.set_start_time_ns(s.start_time_ns);
        span.set_duration_ns(s.duration_ns);
        span.set_id(s.id);
        span.set_parent_id(s.parent_id);
        span.set_event(s.event_num);
    }

    res_spans.write_to_bytes().unwrap()
}

pub fn serialize_fbs_num_event(spans: &[Span]) -> Vec<u8> {
    let mut fbb = flatbuffers::FlatBufferBuilder::new_with_capacity(4096);

    let mut fbs_spans = Vec::with_capacity(spans.len());
    let mut properties_vec = Vec::with_capacity(64);

    for s in spans {
        for p in &s.properties {
            let key = fbb.create_vector_direct(p.key.as_bytes());
            let value = fbb.create_vector_direct(p.value.as_bytes());
            properties_vec.push(tracing_fbs::Property::create(
                &mut fbb,
                &tracing_fbs::PropertyArgs {
                    key: Some(key),
                    value: Some(value),
                },
            ))
        }

        let properties = fbb.create_vector(&properties_vec);
        properties_vec.clear();
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
