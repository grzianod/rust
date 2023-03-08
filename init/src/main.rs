struct Test(i32);
impl Drop for Test {    //drop è un tratto distruttore che il compilatore invoca prima di rilasciare effettivamente lo spazio in memoria
    fn drop(&mut self) {
        println!("Destroying Test ({}) at address {:p}", self.0, self);
    }
}

fn main() {
    /* for & destroying sample */
    for i in 0..4 {
        println!("Iteration #{}", i);
        let t = Test(1);
        println!("Created Test({}) at address {:p}", t.0, &t);
        println!("Finishing block...");
    }

    /* String sample */
    let name :&'static str = "Graziano";
    println!("Hello {}!", name);


    /* Vec<T> & addresses sample */
    let mut v: Vec<Test> = Vec::with_capacity(8);
    println!("ptr: {:p}, capacity: {}, len: {}", v.as_ptr(), v.capacity(), v.len());
    v.push(Test(5));
    println!("ptr: {:p}, capacity: {}, len: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    v.push(Test(7));
    println!("ptr: {:p}, capacity: {}, len: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    v.push(Test(1));
    v.push(Test(32));
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    v.push(Test(23));
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    v.shrink_to_fit();
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    v.pop();
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
}
