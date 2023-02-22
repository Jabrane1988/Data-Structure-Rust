pub fn max_subarray(array: &[i32]) -> i32 {
    let mut tmp = vec![0; array.len()];
    tmp[0] = array[0];
    let mut result = tmp[0];

    for i in 1..array.len() {
        if tmp[i-1] > 0 {
            tmp[i] = tmp[i - 1] + array[i]
        } else {
            tmp[i] = array[i]
        }
        result = result.max(tmp[i]);
    }

    result
}

#[test]
fn test_max_subarray(){
    let array = vec![1,0,5,8];
    assert_eq!(max_subarray(&array), 14);

    let array2 = vec![-3, -1, -8, -2]; // -1
    assert_eq!(max_subarray(&array2), -1);

    let array3 = vec![-4, 3, -2, 5, -3];
    assert_eq!(max_subarray(&array3), 6);
}