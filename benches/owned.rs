#![cfg(feature = "std")]
#![feature(test)]

extern crate sval;
extern crate test;

use sval::value;

use test::{
    black_box,
    Bencher,
};

#[bench]
fn collect_primitive(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::from_value(1);

        black_box(value);
    })
}

#[bench]
fn collect_primitive_string(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::from_value("A string");

        black_box(value);
    })
}

#[bench]
fn collect_complex(b: &mut Bencher) {
    struct Map;

    impl value::Value for Map {
        fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
            stream.map_begin(None)?;

            stream.map_key(1)?;

            stream.map_value_begin()?.map_begin(None)?;

            stream.map_key(2)?;

            stream.map_value(42)?;

            stream.map_end()?;

            stream.map_end()
        }
    }

    b.iter(|| {
        let value = value::OwnedValue::from_value(Map);

        black_box(value);
    });
}