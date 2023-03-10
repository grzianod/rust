#[derive(Debug)]

struct Test(i32);
impl Drop for Test {    //drop è un tratto distruttore che il compilatore invoca prima di rilasciare effettivamente lo spazio in memoria
    fn drop(&mut self) {
        println!("Destroying Test ({}) @ {:p}", self.0, self);
    }
}

//fn alfa(t: Test) il possesso della variabile passa al parametro della funzione alfa11
//fn alfa(t: &Test) verrebbe passato un riferimento senza possesso in sola lettura
//fn alfa(t: &mut Test) verrebbe passato un riferimento senza possesso in scrittura
fn alfa(t: &mut Test) {
    println!("Invoked alfa() on Test({}) @ {:p}", t.0, t);
    t.0 = t.0+1;
    println!("Returning from alfa");
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
    // let mut v: Vec<Test> = Vec::with_capacity(8);
    let mut v: Vec<Test> = Vec::new();
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
    println!("Shrinking vector...");
    v.shrink_to_fit();
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    println!("Popping...");
    v.pop(); //se il valore estratto fosse stato assegnato ad una variabile l'oggetto non verrebbe rimosso e distrutto dallo stack...
    println!("ptr: {:p}, capacity: {}, size: {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);
    println!("Terminating... ");

    /* Movement samples */
    let mut t = Test(12);
    println!("Creating Test ({}) @ {:p}", t.0, &t);
    alfa(&mut t); //alfa diviene il nuovo possessore di t il cui valore viene mosso nel parametro della funzione
    println!("Ending...");

}
