const SSZ_CHUNK_SIZE: usize = 128;
const HASHSIZE: usize = 32;

pub trait TreeHash {
    fn tree_hash(&self) -> Vec<u8>;
}

/// Returns a 32 byte hash of 'list' - a vector of byte vectors.
/// Note that this will consume 'list'.
pub fn merkle_hash(list: &mut Vec<Vec<u8>>) -> Vec<u8> {
    // flatten list
    let (chunk_size, mut data) = list_to_blob(list);

    // get data_len as bytes. It will hashed will the merkle root
    let dlen = list.len() as u64;
    let data_len_bytes = &mut dlen.tree_hash();
    data_len_bytes.resize(32, 0);

    // merklize
    let mut mhash = hash_level(&mut data, chunk_size);
    while mhash.len() > HASHSIZE {
        mhash = hash_level(&mut mhash, HASHSIZE);
    }

    mhash.append(data_len_bytes);
    mhash.as_slice().tree_hash()
}

/// Takes a flat vector of bytes. It then hashes 'chunk_size * 2' slices into
/// a byte vector of hashes, divisible by HASHSIZE
fn hash_level(data: &mut Vec<u8>, chunk_size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for two_chunks in data.chunks(chunk_size * 2) {
        if two_chunks.len() == chunk_size && data.len() > chunk_size {
            // if there is only one chunk here, hash it with a zero-byte
            // SSZ_CHUNK_SIZE vector
            let mut c = two_chunks.to_vec();
            c.append(&mut vec![0; SSZ_CHUNK_SIZE]);
            result.append(&mut c.as_slice().tree_hash());
        } else {
            result.append(&mut two_chunks.tree_hash());
        }
    }

    result
}

fn list_to_blob(list: &mut Vec<Vec<u8>>) -> (usize, Vec<u8>) {
    let chunk_size = if list.is_empty() {
        SSZ_CHUNK_SIZE
    } else if list[0].len() < SSZ_CHUNK_SIZE {
        let items_per_chunk = SSZ_CHUNK_SIZE / list[0].len();
        items_per_chunk * list[0].len()
    } else {
        list[0].len()
    };

    let mut data = Vec::new();
    if list.is_empty() {
        // handle and empty list
        data.append(&mut vec![0; SSZ_CHUNK_SIZE]);
    } else {
        // just create a blob here; we'll divide into
        // chunked slices when we merklize
        data.reserve(list[0].len() * list.len());
        for item in list.iter_mut() {
            data.append(item);
        }
    }

    (chunk_size, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_hash() {
        let data1 = vec![1; 100];
        let data2 = vec![2; 100];
        let data3 = vec![3; 100];
        let mut list = vec![data1, data2, data3];
        let result = merkle_hash(&mut list);

        //note: should test againt a known test hash value
        assert_eq!(HASHSIZE, result.len());
    }
}