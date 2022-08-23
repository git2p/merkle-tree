/**
* I had a lot of weeks blocked in this
* that day, the whore's day was the day i unlocked this
* */
// Merkle Tree
//                       ------------
//                      | Hash(1, 2) | <- Root Node | 2^0 nodes
//                       ------------
//                      |            |
//                ------              ------
//               |                          |
//          ------------              ------------
//         | Hash(3, 4) |            | Hash(5, 6) | <- Internal Node | 2^1 nodes
//          ------------              ------------
//              |  |                      |  |
//          ----    ----              ----    ----
//         |            |            |            |
//    ----------   ----------   ----------   ----------
//   | Hash(b1) | | Hash(b2) | | Hash(b3) | | Hash(b4) |
//   |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| <- Leaf | 2^2 nodes
//   |  Block1  | |  Block2  | |  Block3  | |  Block4  |
//    ----------   ----------   ----------   ----------
pub mod merkle;
mod tree;
mod utils;
