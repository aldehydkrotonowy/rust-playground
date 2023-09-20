pub fn sort(arr: &mut Vec<i32>) {
  mergesort(arr, 0, arr.len() - 1);
}

fn mergesort(arr: &mut Vec<i32>, s: usize, e: usize) {
  if s < e {
    let m = s + (e - s) / 2;
    mergesort(arr, s, m);
    mergesort(arr, m + 1, e);
    merge(arr, s, m, e);
  }
}

fn merge(arr: &mut Vec<i32>, s: usize, m: usize, e: usize) {
  let mut i: usize = s;
  let mut j: usize = m + 1;
  println!(
    "Inside. Array is {:?}, | s:{:?}, m:{:?}, e:{:?} |",
    arr, s, m, e
  );
  let mut aux: Vec<i32> = Vec::new();
  while i <= m && j <= e {
    if arr[i] <= arr[j] {
      aux.push(arr[i]);
      i = i + 1;
    } else {
      aux.push(arr[j]);
      j = j + 1;
    }
  }
  println!("aux1: {:?}, cond:{:?}", aux, i <= m);
  while i <= m {
    aux.push(arr[i]);
    i += 1;
  }
}

#[cfg(test)]
mod test {

  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_merge_in_order() {
    let v: Vec<i32> = vec![1, 3, 6, 11];
    let mut test_arr: Vec<i32> = Vec::new();
    test_arr.push(1);
    test_arr.push(3);
    test_arr.push(6);
    test_arr.push(11);

    let mid: usize = 0 + (3 - 0) / 2;

    merge(&mut test_arr, 0, mid, 3);

    for (pos, el) in test_arr.iter().enumerate() {
      assert_eq!(*el, v[pos]);
    }
  }

  #[test]
  fn test_merge_in_order_2_order() {
    let v: Vec<i32> = vec![1, 3, 6, 11, 12, 15, 28];
    let mut test_arr: Vec<i32> = Vec::new();
    test_arr.push(1);
    test_arr.push(3);
    test_arr.push(6);
    test_arr.push(11);
    test_arr.push(12);
    test_arr.push(15);
    test_arr.push(28);

    let mid: usize = 0 + (3 - 0) / 2;

    merge(&mut test_arr, 0, mid, 3);

    for (pos, el) in test_arr.iter().enumerate() {
      assert_eq!(*el, v[pos]);
    }
  }
}
