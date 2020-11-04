use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct Span {
    pub start_time_ns: u64,
    pub duration_ns: u64,
    pub id: u64,
    pub parent_id: u64,
    pub event_str: String,
    pub event_num: u64,
    pub properties: Vec<Property>,
}

pub struct Property {
    pub key: String,
    pub value: String,
}

fn rand_timestamp_ns() -> u64 {
    rand::thread_rng().gen_range(1600000000_000_000_000, 1700000000_000_000_000)
}

fn rand_duration_ns() -> u64 {
    rand::thread_rng().gen_range(0, 1_000_000_000)
}

fn rand_id() -> u64 {
    rand::thread_rng().gen()
}

fn rand_event_num() -> u64 {
    rand::thread_rng().gen_range(1000, 10000)
}

fn rand_event_str() -> String {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(10, 20);
    rng.sample_iter(&Alphanumeric).take(len).collect()
}

fn rand_property() -> Property {
    let mut rng = rand::thread_rng();
    let key_len = rng.gen_range(5, 10);
    let value_len = rng.gen_range(5, 30);

    Property {
        key: rng.sample_iter(&Alphanumeric).take(key_len).collect(),
        value: rng.sample_iter(&Alphanumeric).take(value_len).collect(),
    }
}

fn rand_properties() -> Vec<Property> {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(1.0 / 10.0) {
        vec![rand_property()]
    } else {
        vec![]
    }
}

fn rand_span() -> Span {
    Span {
        start_time_ns: rand_timestamp_ns(),
        duration_ns: rand_duration_ns(),
        id: rand_id(),
        parent_id: rand_id(),
        event_str: rand_event_str(),
        event_num: rand_event_num(),
        properties: rand_properties(),
    }
}

pub fn rand_spans(count: usize) -> Vec<Span> {
    let mut v = Vec::with_capacity(count);

    for _ in 0..count {
        v.push(rand_span())
    }

    v
}
