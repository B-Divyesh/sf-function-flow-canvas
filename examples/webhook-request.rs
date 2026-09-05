// Bundled local sample used by `ffc --demo`.
#![allow(dead_code)]
struct Bytes;
struct HeaderMap;
struct DomainEvent;
struct Order;
struct OrderId(u64);

fn route_webhook(_handler: fn(Bytes)) {}

fn receive_webhook(body: Bytes) {
    let event = decode_event(body);
    persist_order(to_order(event));
}

fn verify_signature(_headers: HeaderMap) -> bool {
    true
}

fn decode_event(_body: Bytes) -> DomainEvent {
    DomainEvent
}

fn persist_order(_order: Order) -> OrderId {
    OrderId(42)
}

fn to_order(_event: DomainEvent) -> Order {
    Order
}

fn main() {}
