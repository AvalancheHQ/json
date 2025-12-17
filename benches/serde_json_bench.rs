use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    email: String,
    address: Address,
    phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

fn sample_person() -> Person {
    Person {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            zip: "62701".to_string(),
        },
        phones: vec![
            "+1-555-1234".to_string(),
            "+1-555-5678".to_string(),
        ],
    }
}

fn sample_json_str() -> &'static str {
    r#"{
        "name": "John Doe",
        "age": 30,
        "email": "john.doe@example.com",
        "address": {
            "street": "123 Main St",
            "city": "Springfield",
            "state": "IL",
            "zip": "62701"
        },
        "phones": ["+1-555-1234", "+1-555-5678"]
    }"#
}

fn complex_json_str() -> &'static str {
    r#"{
        "users": [
            {
                "id": 1,
                "name": "Alice Smith",
                "email": "alice@example.com",
                "active": true,
                "balance": 1234.56,
                "tags": ["premium", "verified"]
            },
            {
                "id": 2,
                "name": "Bob Jones",
                "email": "bob@example.com",
                "active": false,
                "balance": 789.12,
                "tags": ["standard"]
            }
        ],
        "metadata": {
            "count": 2,
            "timestamp": "2024-01-01T00:00:00Z",
            "version": "1.0"
        }
    }"#
}

fn bench_serialize_struct(c: &mut Criterion) {
    let person = sample_person();
    c.bench_function("serialize_struct", |b| {
        b.iter(|| serde_json::to_string(&person).unwrap())
    });
}

fn bench_serialize_struct_pretty(c: &mut Criterion) {
    let person = sample_person();
    c.bench_function("serialize_struct_pretty", |b| {
        b.iter(|| serde_json::to_string_pretty(&person).unwrap())
    });
}

fn bench_serialize_value(c: &mut Criterion) {
    let value = json!({
        "name": "John Doe",
        "age": 30,
        "email": "john.doe@example.com",
        "active": true,
        "balance": 1234.56
    });
    c.bench_function("serialize_value", |b| {
        b.iter(|| serde_json::to_string(&value).unwrap())
    });
}

fn bench_deserialize_struct(c: &mut Criterion) {
    let json_str = sample_json_str();
    c.bench_function("deserialize_struct", |b| {
        b.iter(|| {
            let _: Person = serde_json::from_str(json_str).unwrap();
        })
    });
}

fn bench_deserialize_value(c: &mut Criterion) {
    let json_str = sample_json_str();
    c.bench_function("deserialize_value", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(json_str).unwrap();
        })
    });
}

fn bench_deserialize_complex(c: &mut Criterion) {
    let json_str = complex_json_str();
    c.bench_function("deserialize_complex", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(json_str).unwrap();
        })
    });
}

fn bench_serialize_bytes(c: &mut Criterion) {
    let person = sample_person();
    c.bench_function("serialize_to_vec", |b| {
        b.iter(|| serde_json::to_vec(&person).unwrap())
    });
}

fn bench_deserialize_bytes(c: &mut Criterion) {
    let json_bytes = sample_json_str().as_bytes();
    c.bench_function("deserialize_from_slice", |b| {
        b.iter(|| {
            let _: Person = serde_json::from_slice(json_bytes).unwrap();
        })
    });
}

fn bench_json_macro(c: &mut Criterion) {
    c.bench_function("json_macro", |b| {
        b.iter(|| {
            json!({
                "name": "John Doe",
                "age": 30,
                "email": "john.doe@example.com",
                "address": {
                    "street": "123 Main St",
                    "city": "Springfield"
                },
                "phones": ["+1-555-1234", "+1-555-5678"]
            })
        })
    });
}

fn bench_value_indexing(c: &mut Criterion) {
    let value: Value = serde_json::from_str(sample_json_str()).unwrap();
    c.bench_function("value_indexing", |b| {
        b.iter(|| {
            let _ = &value["name"];
            let _ = &value["address"]["city"];
            let _ = &value["phones"][0];
        })
    });
}

fn bench_number_parsing(c: &mut Criterion) {
    let numbers = r#"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 123.456, 789.012, -42, -99.99]"#;
    c.bench_function("number_parsing", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(numbers).unwrap();
        })
    });
}

fn bench_string_escaping(c: &mut Criterion) {
    let text_with_escapes = json!({
        "text": "Line 1\nLine 2\tTabbed\r\nWindows line\u{0000}Null char\"Quoted\""
    });
    c.bench_function("string_escaping", |b| {
        b.iter(|| serde_json::to_string(&text_with_escapes).unwrap())
    });
}

fn bench_large_array_serialize(c: &mut Criterion) {
    let large_array: Vec<i32> = (0..1000).collect();
    c.bench_function("large_array_serialize", |b| {
        b.iter(|| serde_json::to_string(&large_array).unwrap())
    });
}

fn bench_large_array_deserialize(c: &mut Criterion) {
    let large_array: Vec<i32> = (0..1000).collect();
    let json_str = serde_json::to_string(&large_array).unwrap();
    c.bench_function("large_array_deserialize", |b| {
        b.iter(|| {
            let _: Vec<i32> = serde_json::from_str(&json_str).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_serialize_struct,
    bench_serialize_struct_pretty,
    bench_serialize_value,
    bench_deserialize_struct,
    bench_deserialize_value,
    bench_deserialize_complex,
    bench_serialize_bytes,
    bench_deserialize_bytes,
    bench_json_macro,
    bench_value_indexing,
    bench_number_parsing,
    bench_string_escaping,
    bench_large_array_serialize,
    bench_large_array_deserialize,
);

criterion_main!(benches);
