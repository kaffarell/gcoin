use md5;

pub fn hash_md5<T: AsRef<[u8]>>(t: T) -> String {
    let digest = md5::compute(t);
    return format!("{:x}", digest);
}