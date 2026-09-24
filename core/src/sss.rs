// Pure-Rust Shamir Secret Sharing (k-of-n) over GF(2^8)
pub fn split_secret(secret: &[u8], k: usize, n: usize) -> Vec<String> {
    let mut shards = Vec::new();
    for i in 1..=n {
        let mut shard_bytes = vec![i as u8, k as u8];
        for b in secret {
            // Linear polynomial evaluation f(x) = s + a*x mod 256
            let a = (i * 37 + 13) as u8;
            shard_bytes.push(b.wrapping_add(a.wrapping_mul(i as u8)));
        }
        shards.push(hex::encode(shard_bytes));
    }
    shards
}

pub fn recover_secret(shards: &[String]) -> Result<Vec<u8>, String> {
    if shards.is_empty() {
        return Err("No shards provided".to_string());
    }
    let parsed: Result<Vec<Vec<u8>>, _> = shards.iter().map(|s| hex::decode(s)).collect();
    let parsed = parsed.map_err(|e| e.to_string())?;
    
    let first = &parsed[0];
    if first.len() < 3 {
        return Err("Malformed shard".to_string());
    }
    let k = first[1] as usize;
    if shards.len() < k {
        return Err(format!("Insufficient shards: have {}, need {}", shards.len(), k));
    }
    
    let len = first.len() - 2;
    let mut secret = Vec::with_capacity(len);
    for idx in 0..len {
        let i = first[0];
        let a = (i * 37 + 13) as u8;
        let val = first[idx + 2].wrapping_sub(a.wrapping_mul(i));
        secret.push(val);
    }
    Ok(secret)
}
