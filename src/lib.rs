use anyhow::Error;
use itertools::izip;
use once_cell::sync::OnceCell;
use snarkvm::circuit::IndexMap;
use snarkvm::prelude::to_bits::ToBits;

use snarkvm::prelude::{
    Address, Field, Group, Identifier, Literal, Network, Plaintext, Testnet3, U128, U16, U32, U64,
    U8,
};
use std::str::FromStr;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub extern "C" fn rust_function() {
    // ваш код здесь
    println!("Called rust_function from Rust!");
    let hash = hash_struct(
        vec!["id", "addr"],
        vec!["field", "address"],
        vec![
            "123field",
            "aleo1r3qlsxnuux6rkrhk24rktdtzu7kjr3c2fw5fvtp6a9dwghe0xgzs9c2nhu",
        ],
    )
    .unwrap();
    println!("{:?}", hash);
}

pub(crate) fn hash_struct(
    names: Vec<&str>,
    types: Vec<&str>,
    value: Vec<&str>,
) -> Result<Field<Testnet3>, Error> {
    let value = Plaintext::<Testnet3>::Struct(
        IndexMap::from_iter(izip!(names, types, value).map(|(n, t, v)| {
            (
                Identifier::from_str(n).unwrap(),
                match t {
                    "u128" => Plaintext::<Testnet3>::Literal(
                        Literal::U128(U128::new(v.parse::<u128>().unwrap())),
                        OnceCell::new(),
                    ),
                    "u64" => Plaintext::<Testnet3>::Literal(
                        Literal::U64(U64::new(v.parse::<u64>().unwrap())),
                        OnceCell::new(),
                    ),
                    "u32" => Plaintext::<Testnet3>::Literal(
                        Literal::U32(U32::new(v.parse::<u32>().unwrap())),
                        OnceCell::new(),
                    ),
                    "u16" => Plaintext::<Testnet3>::Literal(
                        Literal::U16(U16::new(v.parse::<u16>().unwrap())),
                        OnceCell::new(),
                    ),
                    "u8" => Plaintext::<Testnet3>::Literal(
                        Literal::U8(U8::new(v.parse::<u8>().unwrap())),
                        OnceCell::new(),
                    ),
                    "address" => Plaintext::<Testnet3>::Literal(
                        Literal::Address(Address::<Testnet3>::from_str(v).unwrap()),
                        OnceCell::new(),
                    ),
                    "field" => Plaintext::<Testnet3>::Literal(
                        Literal::Field(Field::<Testnet3>::from_str(v).unwrap()),
                        OnceCell::new(),
                    ),
                    "group" => Plaintext::<Testnet3>::Literal(
                        Literal::Group(Group::<Testnet3>::from_str(v).unwrap()),
                        OnceCell::new(),
                    ),
                    _ => Plaintext::<Testnet3>::Literal(
                        Literal::U128(U128::new(v.parse::<u128>().unwrap())),
                        OnceCell::new(),
                    ),
                },
            )
        })),
        OnceCell::new(),
    );
    Testnet3::hash_bhp256(&*value.to_bits_le())
}

fn main() {
    // The output should match this Leo program return value
    // ======================================================================================
    //     program leo_hash_test.aleo {
    //
    //     struct Test {
    //         id: field,
    //         addr: address
    //     }
    //     transition main(public a: u32, b: u32) -> field {
    //         let c: Test = Test {
    //             id: 123field,
    //             addr: aleo1r3qlsxnuux6rkrhk24rktdtzu7kjr3c2fw5fvtp6a9dwghe0xgzs9c2nhu
    //         };
    //         return BHP256::hash_to_field(c);
    //     }
    // }
    // ======================================================================================
    let hash = hash_struct(
        vec!["id", "addr"],
        vec!["field", "address"],
        vec![
            "123field",
            "aleo1r3qlsxnuux6rkrhk24rktdtzu7kjr3c2fw5fvtp6a9dwghe0xgzs9c2nhu",
        ],
    )
    .unwrap();
    println!("{:?}", hash);
}
