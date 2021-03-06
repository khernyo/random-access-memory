#[macro_use]
extern crate quickcheck;
extern crate random_access_memory as ram;

use self::Op::*;
use quickcheck::{Arbitrary, Gen};
use std::u8;

const MAX_FILE_SIZE: usize = 5 * 10; // 5mb

#[derive(Clone, Debug)]
enum Op {
  Read { offset: usize, length: usize },
  Write { offset: usize, data: Vec<u8> },
}

impl Arbitrary for Op {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let offset: usize = g.gen_range(0, MAX_FILE_SIZE);
    let length: usize = g.gen_range(0, MAX_FILE_SIZE / 3);

    if g.gen::<bool>() {
      Read { offset, length }
    } else {
      let mut data = Vec::with_capacity(length);
      for _ in 0..length {
        data.push(u8::arbitrary(g));
      }
      Write { offset, data }
    }
  }
}

quickcheck! {
  fn implementation_matches_model(ops: Vec<Op>) -> bool {
    let mut implementation = ram::RandomAccessMemory::new(10);
    let mut model = vec![];

    for op in ops {
      match op {
        Read { offset, length } => {
          let end = offset + length;
          if model.len() >= end {
            assert_eq!(
              &*implementation.read(offset, length).expect("Reads should be successful."),
              &model[offset..end]
            );
          } else {
            assert!(implementation.read(offset, length).is_err());
          }
        },
        Write { offset, ref data } => {
          let end = offset + data.len();
          if model.len() < end {
            model.resize(end, 0);
          }
          implementation.write(offset, &*data).expect("Writes should be successful.");
          model[offset..end].copy_from_slice(data);
        },
      }
    }
    true
  }
}
