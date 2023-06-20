
#[warn(dead_code)]
pub fn add(a: i32, b:i32) -> i32 {
    a + b 
}

fn bubble_sort(arr: &mut [i32]) {

    let n = arr.len();

    for (i, j) in Iterator::enumerate(0..n) {

        for (k, l) in Iterator::enumerate(i + 1..n) {

            if arr[i] > arr[l] {

                arr.swap(i, l);

            }

        }

    }

}



pub fn sort_maopao(a: &mut [i32]) {
    let aLen = a.len();
    if aLen <= 1 {
        return
    }
    for i in 0..aLen - 1 {
        for j in i+1..aLen  {
            if a[i] > a[j] {
                // let temp = a[i];
                // a[i] = a[j];
                // a[j] = temp;
                a.swap(i, j)
            }
        }
    }
}

#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn t1() {
        let r = add(1,2);
        // println("res={}", r);
        assert_eq!(3, r)
    }

    #[test]
    fn t2() {
        let mut a = [1,4,2,6,3,7,23,23423,99];
        let aLen = a.len();
        let a_slice = &mut a[0..=aLen-2];
        println!("before {:?}", a_slice);
        sort_maopao(a_slice);
        println!(" after {:?}", a_slice);
    }

    #[test]
    fn t3() {
        let mut a = [1,2,3,8,7,6];
        let br: &mut [i32] = &mut a[0..6];
        let r = bubble_sort(br);
        print!("结果：{:#?}", br);
    }
}