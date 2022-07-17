use rand::{distributions::Alphanumeric, Rng};

/**
    Generate unique id

    ```
    use new_york_utils::make_id;

    let id = make_id(6);
    assert_eq!(id.len(), 6)
    ```
*/
pub fn make_id(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
