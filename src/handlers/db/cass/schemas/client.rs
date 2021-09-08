


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



// #[derive(Serialize, Deserialize,Clone, Debug, PartialEq)]
pub struct Client {
    pub id: Uuid,
    pub user_id: i32,
    pub token: String,
    pub address: SocketAddress,
    pub time: u64,
}






impl Client{
    pub async fn first(session: Arc<CassSession>) -> Self{
        Client{id: Uuid::new_v4(), user_id: 0 , time_now: 0 , message: String::from("")}
    }
    
    pub async fn all(session: Arc<CassSession>) -> Self{
        Client{id: Uuid::new_v4(), user_id: 0 , time_now: 0 , message: String::from("")}
    }
    
    pub async fn first_by_id(session: Arc<CassSession>) -> Self{
        Client{id: Uuid::new_v4(), user_id: 0 , time_now: 0 , message: String::from("")}
    }

    pub async fn last(session: Arc<CassSession>) -> Self{
        Client{id: Uuid::new_v4(), user_id: 0 , time_now: 0 , message: String::from("")}
    }


    fn insert(&self) -> QueryValues{
        query_values!("id" => self.id, "user_id" => self.user_id , "time_now" => self.time_now, "message" => self.message.clone())}



    pub async fn init(session: Arc<CassSession>){
        let create_client_table = "CREATE TABLE IF NOT EXISTS supapp.client (id UUID , user_id int, time_now timestamp , message text ,PRIMARY KEY((id,user_id)));";
        session.query(create_client_table).await.expect("⚠️ client table creation error");
    }



    pub async fn save(&self, session: Arc<CassSession>){
        let insert_client_cql = "INSERT INTO supapp.client (id, user_id , time_now , message) VALUES (?, ?, ?, ?, ?, toUnixTimestamp(now()))";
        let values = self.insert();
        session.query_with_values(insert_client_cql, values).await.expect("⚠️ client column family insertion error");
    }
}

