extern crate chrono;

use std::collections::HashMap;

fn main() {
    println!("★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★　★");


    //-----------------------------------------------------------
    let w_map = get_map();


    for (k, v) in &w_map {
        println!("key={} value={}", k, v);
    }

    let w_katie = w_map.get("Katie");
    println!("w_katie={:?}", w_katie);

    //ベクタのi32を合計する---------------------------------------------
    let n3: usize = 16;
    let mut v3: Vec<i32> = Vec::new();
    for i3 in 0..n3 {
        v3.push(i3 as i32);
    }
    let w_aaaaa3 = add_vec_int( n3, v3 );
    println!("合計は{}", w_aaaaa3);

    //ベクタ文字列を連結して返却する--------------------------------------
    // let mut w_s3 = Vec::new();
    // w_s3.push("南");
    // w_s3.push("沢");
    // w_s3.push("郁");
    // w_s3.push("男");
    let w_s3 = vec!["南","沢","郁","男"];
    let w_vv3 = add_vec_st(w_s3);
    println!("w_vv3の答えは{}", w_vv3);




}


//HashMapを返却する
fn get_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    map.insert("Daniel", "a798-1364");
    map.insert("Ashley", "a645-7689");
    map.insert("Katie", "a435-8291");
    map.insert("Robert", "a956-1745");

    map
}

 //ベクタの要素文字列を連結する
fn add_vec_st(v: Vec<&str> ) -> String {
    let mut s11 = String::new();
    for vs1 in v.iter(){
        s11.push_str(vs1);
    }
    s11
}

//i32のベクタ型の要素を合計して返却する
fn add_vec_int( n: usize, v: Vec<i32> ) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s = s + v[i];
    }
    s
}


