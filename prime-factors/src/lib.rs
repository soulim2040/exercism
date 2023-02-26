
pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut div = 2;

    let mut mn = n;
    while mn > 1 {
        if mn % div == 0 {
            result.push(div);
            mn /= div;
        }else{
           div += 1; 
        }
    }

    result
}
