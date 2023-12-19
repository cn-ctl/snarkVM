// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::msm::*;
use snarkvm_curves::{
    bls12_377::{Fr, G1Projective, FqParameters, Bls12_377G1Parameters},
    traits::{AffineCurve, ProjectiveCurve}, templates::short_weierstrass_jacobian::Affine,
};
use snarkvm_fields::{PrimeField, Zero, Fp384};
use snarkvm_utilities::{
    rand::{TestRng, Uniform},
    BitIteratorBE, BigInteger384, BigInteger256, 
};
use std::marker::PhantomData;
fn naive_variable_base_msm<G: AffineCurve>(
    bases: &[G],
    scalars: &[<G::ScalarField as PrimeField>::BigInteger],
) -> G::Projective {
    let mut acc = G::Projective::zero();

    for (base, scalar) in bases.iter().zip(scalars.iter()) {
        acc += base.mul_bits(BitIteratorBE::new(*scalar));
    }
    acc
}

#[test]
fn variable_base_test_with_bls12() {
    const SAMPLES: usize = 1 << 10;
    for _ in 0..10{
        let mut rng = TestRng::default();
        let v = (0..SAMPLES).map(|_| Fr::rand(&mut rng).to_bigint()).collect::<Vec<_>>();
        let g = (0..SAMPLES).map(|_| G1Projective::rand(&mut rng).to_affine()).collect::<Vec<_>>();

        let naive = naive_variable_base_msm(g.as_slice(), v.as_slice());
        let fast = VariableBase::msm(g.as_slice(), v.as_slice());

        assert_eq!(naive.to_affine(), fast.to_affine());
    }
    
}

#[test]
fn xxxxxx(){
    let str = "110101011100100011111011011111000000011110111001010010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    println!("{}",str.len());
}


#[test]
fn variable_base_test_with_bls12_unequal_numbers() {
    const SAMPLES: usize = 1 << 10;

    let mut rng = TestRng::default();

    let v = (0..SAMPLES - 100).map(|_| Fr::rand(&mut rng).to_bigint()).collect::<Vec<_>>();
    let g = (0..SAMPLES).map(|_| G1Projective::rand(&mut rng).to_affine()).collect::<Vec<_>>();

    let naive = naive_variable_base_msm(g.as_slice(), v.as_slice());
    let fast = VariableBase::msm(g.as_slice(), v.as_slice());

    assert_eq!(naive.to_affine(), fast.to_affine());
}

#[test]
fn double_in_place_test(){
    //在下面填写x,y,z坐标
    let (x,y,z) = (1,2,3);

    let (x,y,z) = (Fp384::from_bigint(BigInteger384::from(x)).unwrap(),Fp384::from_bigint(BigInteger384::from(y)).unwrap(),Fp384::from_bigint(BigInteger384::from(z)).unwrap());
    println!("x:{x:?}");
    println!("y:{y:?}");
    println!("z:{z:?}");
    let mut g = G1Projective::new(x,y,z);
    println!("{g:?}");
    g.double_in_place();
    println!("");
    println!("{g:?}");
}

#[test]
fn double_in_place_test_random(){
    let mut rng = TestRng::default();
    
    let mut g = G1Projective::rand(&mut rng);
    println!("{g:?}");
    g.double_in_place();
    println!("");
    println!("{g:?}");
}

#[test]
fn mul_bits_test(){
    //t填scalar的数值
    let t: u64 = 1;
    let scalar = BigInteger256::from(t);
    println!("t:{t:?}\nscalar:{scalar:?}");
    //在下面填写x,y坐标
    let (x,y) = (1,2);

    let (x,y) = (Fp384::from_bigint(BigInteger384::from(x)).unwrap(),Fp384::from_bigint(BigInteger384::from(y)).unwrap());
    println!("x:{x:?}");
    println!("y:{y:?}");
    let base: Affine<Bls12_377G1Parameters> = Affine::new(x,y,false);
    println!("base:{base:?}");

    let result = base.mul_bits(BitIteratorBE::new(scalar));

    println!("result:{result:?}");
}

#[test]
fn mul_bits_test_random(){
    let mut rng = TestRng::default();
    
    let scalar = Fr::rand(&mut rng).to_bigint();
    let base = G1Projective::rand(&mut rng).to_affine();
    println!("{scalar:?}\n\n{base:?}\n");

    let result = base.mul_bits(BitIteratorBE::new(scalar));

    println!("{result:?}");
}

#[test]
fn add_assign_test(){
    //在下面填写x,y,z坐标
    let (x,y,z) = (1,2,3);

    let (x,y,z) = (Fp384::from_bigint(BigInteger384::from(x)).unwrap(),Fp384::from_bigint(BigInteger384::from(y)).unwrap(),Fp384::from_bigint(BigInteger384::from(z)).unwrap());
    println!("x:{x:?}");
    println!("y:{y:?}");
    println!("z:{z:?}");
    let mut a = G1Projective::new(x,y,z);    

    
    //在下面填写另一组x,y,z坐标
    let (x,y,z) = (1,2,3);

    let (x,y,z) = (Fp384::from_bigint(BigInteger384::from(x)).unwrap(),Fp384::from_bigint(BigInteger384::from(y)).unwrap(),Fp384::from_bigint(BigInteger384::from(z)).unwrap());
    println!("x:{x:?}");
    println!("y:{y:?}");
    println!("z:{z:?}");
    let b = G1Projective::new(x,y,z);    

    println!("a:{a:?}\n\nb:{b:?}\n");
    a+=b;
    println!("a:{a:?}");
}