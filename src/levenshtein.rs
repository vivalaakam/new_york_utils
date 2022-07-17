use crate::error::UtilsResult;
use crate::matrix::Matrix;

pub fn levenshtein<T>(vec_1: Vec<T>, vec_2: Vec<T>) -> UtilsResult<i32>
where
    T: Into<String> + PartialEq,
{
    if vec_1.len() == 0 {
        return Ok(vec_2.len() as i32);
    }
    if vec_2.len() == 0 {
        return Ok(vec_1.len() as i32);
    }

    let vec_1_len = vec_1.len() + 1;
    let vec_2_len = vec_2.len() + 1;

    let mut matrix = Matrix::new(vec_2_len, vec_1_len);

    for i in 0..vec_2_len {
        let _ = matrix.set(i, 0, i as i32);
    }

    for j in 0..vec_1_len {
        let _ = matrix.set(0, j, j as i32);
    }

    for i in 1..vec_2_len {
        for j in 1..vec_1_len {
            let cost = if vec_2[i - 1] == vec_1[j - 1] { 0 } else { 1 };
            let val = *[
                matrix.get(i - 1, j - 1).unwrap() + cost,
                matrix.get(i, j - 1).unwrap() + 1,
                matrix.get(i - 1, j).unwrap() + 1,
            ]
            .iter()
            .min()
            .unwrap();

            let _ = matrix.set(i, j, val);
        }
    }

    matrix.get(vec_2.len(), vec_1.len())
}

#[cfg(test)]
mod tests {
    use crate::levenshtein::levenshtein;

    #[test]
    fn it_works() {
        let distance = levenshtein(
            vec!["k", "i", "t", "t", "e", "n"],
            vec!["s", "m", "i", "t", "t", "e", "n"],
        );

        assert_eq!(distance.unwrap(), 2);

        let distance = levenshtein(
            vec!["k", "i", "t", "t", "e", "n"],
            vec!["f", "i", "t", "t", "i", "n", "g"],
        );

        assert_eq!(distance.unwrap(), 3);
    }
}
