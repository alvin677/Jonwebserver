use actix_web::{App, HttpServer};
use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls::server::{ResolvesServerCert, ResolvesServerCertUsingSni};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;
use evmap::Options;

struct DynamicCertResolver(ResolvesServerCertUsingSni);

impl DynamicCertResolver {
    fn new() -> Self {
        let mut resolver = ResolvesServerCertUsingSni::new();
        // Add default or initial certificates
        resolver.add("example.com", (vec![Certificate(vec![])], PrivateKey(vec![]))).unwrap();
        Self(resolver)
    }
}

impl ResolvesServerCert for DynamicCertResolver {
    // Implement logic to resolve certificates dynamically
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(DynamicCertResolver::new()));
    
    let mut configHTTP = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth();

    HttpServer::new(|| App::new())
        .workers(num_cpus::get() / 2)
        .bind_rustls("0.0.0.0:443", config)?
        .run()
        .await

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
    .workers(num_cpus::get() / 2)  // Uses all CPU cores
    .bind("0.0.0.0:80", configHTTP)?
    .run()
    .await
}

fn dict() {
    let my_map: DashMap<&str, &str> = DashMap::new();

    my_map.insert("key1", "value1");
    my_map.insert("key2", "value2");

    if let Some(value) = my_map.get("key1") {
        println!("Value for 'key1': {}", value);
    }
}

// For moderate reads and writes.
let dashmap: Arc<DashMap<Vec<u8>, DashMap<Vec<u8>, String>>> = Arc::new(DashMap::new());
fn DashMapGet(id: Vec<u8>, map: Arc<DashMap<Vec<u8>, DashMap<Vec<u8>, String>>>) -> Option<DashMap<Vec<u8>, String>> {
    if let Some(value) = map.get(&id) {
        Some(value.clone()) // Cloning ensures the value can be returned
    } else {
        None
    }
}
fn DashMapSet(id: Vec<u8>, inner_map: DashMap<Vec<u8>, String>, map: Arc<DashMap<Vec<u8>, DashMap<Vec<u8>, String>>>) {
    map.insert(id, inner_map);
}

// For frequent reads with occasional writes.
let (read_handle, mut write_handle) = evmap::new::<u8, String>();
fn evmapGet(id: &u8, read_handle: &evmap::ReadHandle<u8, String>) -> Option<Vec<String>> {
    read_handle.get(id).map(|values| values.clone())
}
fn evmapSet(id: u8, val: String, write_handle: &mut evmap::WriteHandle<u8, String>) {
    write_handle.update(id, val);
    write_handle.refresh(); // Ensure the update is visible to readers
}

// For frequent writes and frequent reads.
let counter = AtomicUsize::new(0);
fn AtomicSet(val: usize) {
    counter.fetch_add(val, Ordering::SeqCst);
}
fn AtomicGet() -> usize {
    counter.load(Ordering::SeqCst)
}