use std::collections::HashMap;
pub fn raindrops(n: u32) -> String {
    //3 Pling, 5 Plang, 7 Plong,

    // let mut dic = HashMap::new();
    // dic.insert(3, "Pling");
    // dic.insert(5, "Plang");
    // dic.insert(7, "Plong");
    
    
    // let mut mn = n;
    // let mut result = HashMap::new();

    // let mut keys = dic.keys().collect::<Vec<_>>();
    // keys.sort();

    // let mut vec_str = vec![];
    // for k in keys  {
    //     if mn % k == 0 {
    //         let val = dic[k].to_string();
    //         let count = result.entry(val.clone()).or_insert(1);
    //         if count == &1 {
    //             vec_str.push(val);
    //         }else{
    //             *count += 1;
    //         }
    //         mn = mn / k;
    //     }
    // }

    // if vec_str.len() == 0 {
    //     n.to_string()
    // }else{
    //     vec_str.join("")
    // }

    let mut ret = String::new();

    let mut f = |i, str| {
        if n % i == 0  {
            ret.push_str(str);
        }
    };

    f(3, "Pling");
    f(5, "Plang");
    f(7, "Plong");

    if ret.is_empty(){
        n.to_string()
    }else{
        ret
    }
}
