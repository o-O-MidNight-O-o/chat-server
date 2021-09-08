




/*  

TODO - 

    https://github.com/krojew/cdrs-tokio/blob/master/documentation
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/multiple_thread.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/insert_collection.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/prepare_batch_execute.rs
    https://www.oreilly.com/library/view/cassandra-the-definitive/9781491933657/ch04.html

*/




use crate::handlers::db::cass::establish::CassSession;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::frame::AsBytes;
use cdrs_tokio::types::from_cdrs::FromCdrsByName;
use cdrs_tokio::types::prelude::*;
use cdrs_tokio_helpers_derive::*;
use chrono::prelude::*;







/////////////////////////////////////////////////// CASSANDRA ARCHITECTURE ///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////// https://github.com/krojew/cdrs-tokio/blob/master/type-mapping.md ////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 

// #[derive(Serialize, Deserialize,Clone, Debug, PartialEq)]
pub struct ChatUser {
    pub id: Uuid,
    pub user_id: i32,
    pub friend_id: i32,
    pub time_now: u64,
    pub message: String,
}






impl ChatUser {
    pub async fn first(session: Arc<CassSession>) -> Self{
        ChatUser {id: Uuid::new_v4(), user_id: 0 ,friend_id: 0, time_now: 0 , message: String::from("")}
    }
    
    pub async fn all(session: Arc<CassSession>) -> Self{
        ChatUser {id: Uuid::new_v4(), user_id: 0 ,friend_id: 0, time_now: 0 , message: String::from("")}
    }
    
    pub async fn first_by_id(session: Arc<CassSession>) -> Self{
        ChatUser {id: Uuid::new_v4(), user_id: 0 ,friend_id: 0, time_now: 0 , message: String::from("")}
    }

    pub async fn last(session: Arc<CassSession>) -> Self{
        ChatUser {id: Uuid::new_v4(), user_id: 0 ,friend_id: 0, time_now: 0 , message: String::from("")}
    }


    fn insert(&self) -> QueryValues{
        query_values!("id" => self.id, "user_id" => self.user_id , "friend_id" => self.friend_id ,"time_now" => self.time_now, "message" => self.message.clone())}



    pub async fn init(session: Arc<CassSession>){
        let create_chat_user_table = "CREATE TABLE IF NOT EXISTS supapp.chat_user (id UUID , user_id int, friend_id int, time_now timestamp , message text ,PRIMARY KEY((id,time))WITH CLUSTERING ORDER BY (time ASC));";
        session.query(create_chat_user_table).await.expect("⚠️ chat_user table creation error");
    }



    pub async fn save(&self, session: Arc<CassSession>){
        let insert_chat_user_cql = "INSERT INTO supapp.chat_user (id, user_id , friend_id , time_now , message) VALUES (?, ?, ?, ?, ?, toUnixTimestamp(now()))";
        let values = self.insert();
        session.query_with_values(insert_chat_user_cql, values).await.expect("⚠️chat_user column family insertion error");
    }
}
