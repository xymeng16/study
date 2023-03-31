use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct DataType {
    data: Vec<Move>,
    cursor: usize,
}

// impl Read for DataType {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         // read all of self into buf
//         let encode: Vec<u8> = bincode::serialize(&self.data).unwrap();
//         // what is the buf size?
//         println!("buf.len() = {}, encode.len() = {}, self.cursor = {}", buf.len(), encode.len(), self.cursor);
//
//         buf.copy_from_slice(&encode.as_slice()[self.cursor..(self.cursor + buf.len())]);
//
//         self.cursor += buf.len();
//
//         Ok(buf.len())
//     }
// }
//
// impl Write for DataType {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         todo!()
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         todo!()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: u32,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            direction: Direction::Right,
            distance: 10,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

/// 1. use a wrapper type that implements Serialize and Deserialize
/// 2. to_document then to_writer to serialize
/// 3. from_reader then from_document to deserialize
fn main() {
    let a = data_gen(1000);

    let mut file = File::create("a.bson").unwrap();
    let ll = bson::to_document(&a).unwrap().len();
    let mut buf: Vec<u8> = Vec::with_capacity(50000);
    println!("before: file size {}", buf.len());

    bson::to_document(&a).unwrap().to_writer(&mut buf).unwrap();
    println!("after: file size {}",buf.len());

    // let b = bson::to_bson(&a).unwrap();
    // let mut file = File::open("a.bson").unwrap();
    //
    let deserialized: DataType =
        bson::from_document(bson::Document::from_reader(&mut buf).unwrap()).unwrap();

    println!("deserialized[10] = {:?}", deserialized.data[10]);
}

fn data_gen(num: usize) -> DataType {
    let mut ret = Vec::with_capacity(num);

    for i in 0..num {
        ret.push(Move {
            direction: Direction::Down,
            distance: i as u32,
        });
    }

    DataType {
        data: ret,
        cursor: 0,
    }
}
