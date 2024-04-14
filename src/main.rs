fn binary_search(arr: &mut[i32;20]) -> i32{
    let search_number = 4;
    let mut low:usize = 0;
    let mut high:usize = arr.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        let guess:usize = arr[mid] as usize;
        if guess == search_number {
            return mid as i32;
        }else if guess < search_number {
            low = mid + 1;
        }else {
            high = mid - 1;
        }
    }
    return -1;
}

fn main() {
    let mut arr = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    println!("Бинарный поиск, искомое число под индексом: {}", binary_search(&mut arr));
}