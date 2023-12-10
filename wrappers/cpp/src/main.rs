use pairing_crypto::bbs_bound::ciphersuites::bls12_381_bbs_g1_bls_sig_g2_sha_256::BbsKeyPair;

#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
        fn crypto_test(seed: &[u8], key_info: &[u8]);
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("paring_crypto_cpp/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
        fn invoke_crypto_test(&self, seed: &[u8], key_info: &[u8]);
    }
}

// An iterator over contiguous chunks of a discontiguous file object.
//
// Toy implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

pub fn crypto_test(seed: &[u8], key_info: &[u8]) {
    let x = BbsKeyPair::new(seed, key_info);
    println!("{:?}", x);
}

fn main() {
    const TEST_KEY_GEN_SEED: &[u8; 32] = b"not_A_random_seed_at_Allllllllll";
    const TEST_KEY_INFO: &[u8] = b"test-key-info";
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);

    client.invoke_crypto_test(TEST_KEY_GEN_SEED, TEST_KEY_INFO);
}
