


/*


    https://github.com/wildonion/aravl/blob/master/docs/rust.rules
    https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1
    https://doc.rust-lang.org/std/pin/index.html
    https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
    https://danielkeep.github.io/tlborm/book/
    https://cetra3.github.io/blog/implementing-a-jobq/
    https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
    https://docs.rs/tokio/1.7.1/tokio/sync/index.html
    http://gradebot.org/doc/ipur/trait.html
    https://doc.rust-lang.org/std/sync/struct.Arc.html
    https://doc.rust-lang.org/std/sync/struct.Mutex.html
    


    tokio::spawn() is a multithreaded async task handler
    we can use raw future messages of rust by awaiting on them using tokio runtime 
    or use tokio::spawn() which is multithreaded asynchronous task handler 
    to solve every event future sent by the producer.


    we can’t just pass the receiver between multiple threads cause trait Clone which is a super trait of Copy is not implemented for the receiver thus we can’t clone it to fix the issue cause if a type is Copy its Clone needs to return *self.
    Multiple producer means multiple threads own the receiver and single consumer means only one of them can mutate and get the job or task from the receiver at a time.
    to fix the issue we have to take an atomic reference from the receiver using Arc in order to clone it for passing between multiple threads and for mutating it we have to 
    put it inside a Mutex to insure that only one thread can change the content of the receiver at a time. this is done by waiting on the receiver until a job or task becomes 
    available to the down side of the channel then locking on the receiver to acquire the mutex.
    the receiver of tokio mpsc channel is shareable between tokio::spawn() threads so we don’t need to take an atomic reference and put it inside the Mutex.


    clone data structure if you want to move them between threads so trait Clone must be implemented for them otherwise clone them using Arc.
    thread safe coding is about to putting the shareable receiver (cloned with Arc) inside a Mutex in order to lock on the incoming task from the sender to prevent other threads from mutating the task at a time.
    
    
    live streaming is done using socket, futures, threadpool and mpsc protocol from scratch
    tokio::spawn() is a multithreaded async task handler based on mpsc protocol


    we can't have a clone from the receiver in mpsc protocol to fix the issue cause if a type is Copy it must have Clone also and its Clone needs to return *self
    can't clone a data structure unless the trait Clone is implemented for that otherwise in order to move it between threads we have to clone it using Arc
    every Copy type is also required to be Clone and if T: Copy, x: T, and y: &T, then let x = y.clone(); is equivalent to let x = *y;
    when we derive a Copy implementation Clone is also required cause it's a super trait of Copy.


    if a type imeplements trait Copy means we can clone it (cause trait Clone is a super trait of Copy) and also assign the variable into another one without losing the ownership of our variable


*/



use crate::handlers::db::cass::schemas::from_user::ChatUser;
use crate::handlers::db::cass::establish as cass;
use crate::handlers::db::pg::establish as pg;
use log::{error, info};



pub async fn produce(brokers: &str){


    


    // NOTE - cloning db connections using Arc cause trait Copy and Clone is not implemented for them and they are not Sync and Send and safe to move between threads thus Arc do these things for us
    let pg_pool = pg::connection().await.expect("⚠️ can't establish pg connections"); //-- making postgres pool of connections before the accepting each socket connection
    let cass_session = cass::connection().await.expect("⚠️ can't establish cassandra connection!"); //-- making cassandra pool of connections for selected node
    let pg_pool = pg_pool.clone(); //-- cloning the immutable postgres pool connections to move it into every async task scope and share between tokio::spawn() threads
    let cass_session = cass_session.clone(); //-- cloning the immutable cassandra session so we can share its ownership between multiple threads
    
    


    ChatUser::init(cass_session.clone()).await; //-- it'll create player_data column family if there is not any
    tokio::spawn(async move{
        // TODO - fetching rows from some table
        // ...
        // let mut conn = pg_pool.get().unwrap();
        // let some_id_for_fetch = 23;
        // let fetch_statement = conn.prepare("SELECT some_column FROM some_table WHERE some_thing = $1").unwrap();
        // let rows = conn.query(&fetch_statement, &[&some_id_for_fetch]).unwrap();
        // let device_id: String = rows[0].get(0); //-- getting the first row of the first column
        
    });





}
