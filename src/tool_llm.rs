use openai_func_enums::RunCommand;

use crate::tool::CliCommands;

// impl RunCommand for CliCommands {
//     #[must_use]
//     // #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn run<'life0, 'async_trait>(
//         &'life0 self,
//     ) -> ::core::pin::Pin<
//         Box<
//             dyn ::core::future::Future<
//                     Output = Result<
//                         Option<String>,
//                         Box<dyn std::error::Error + Send + Sync + 'static>,
//                     >,
//                 > + ::core::marker::Send
//                 + 'async_trait,
//         >,
//     >
//     where
//         'life0: 'async_trait,
//         Self: 'async_trait,
//     {
//         todo!()
//     }
// }
