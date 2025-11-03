// use {
//     crate::{
//         api::{native_transfer},
//         context::Context,
//     },
//     anyhow::Result,
//     solana_sdk::{
//         signature::{Keypair, Signature},
//         signer::Signer,
//     },
//     tokio::sync::Semaphore,
//     std::sync::Arc,
//     futures_util::future::join_all,
// };
// use hello_world::Instruction;
//
// const MIN_BALANCE: u64 = 1000000000;
//
// pub async fn distribute_2<'a>(
//     context: Context,
//     count: u64,
// ) -> Vec<Result<Signature>> {
//     let mut ret_value: Vec<Result<Signature>> = vec![];
//
//     let mut accounts = vec![];
//
//     for _ in 0..count {
//         accounts.push(Keypair::new());
//     }
//
//     let semaphore = Arc::new(Semaphore::new(10));
//
//     let mut handles = vec![];
//
//     let ctx = Arc::new(context);
//
//
//     for i in 0..count {
//         //let semaphore = Arc::clone(&semaphore);
//         let permit = semaphore.clone().acquire_owned().await.unwrap();
//
//
//
//         let key = accounts.get(i as usize).unwrap().pubkey().clone();
//
//         let ctx1 = Arc::clone(&ctx);
//         //let ctx = Context::new(context.program_id, context.keypair, context.client).unwrap();
//
//         handles.push(tokio::spawn(async move {
//             let _sig = native_transfer(
//                 ctx1,
//                 MIN_BALANCE,
//                 key,
//             ).await;
//             drop(permit);
//         }));
//
//
//
//
//
//     }
//
//     let results = join_all(handles).await;
//
//     // for mut result in results {
//     //     ret_value.append(&mut result.unwrap());
//     // };
//
//     ret_value
// }
