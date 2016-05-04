#![feature(test)]

extern crate generic_array;
extern crate arraymap;
extern crate arrayvec;
extern crate typenum;
extern crate test;

macro_rules! bench_map {
    {$u:ident, $n:expr, $a:ident, $g:ident, $h:ident, $i:ident} => {
        #[bench]
        fn $a(b: &mut Bencher) {
            let arr = [0u32; $n];
            b.iter(|| black_box(arr).map(|x| x + 1));
        }
   
        #[bench]
        fn $g(b: &mut Bencher) {
            let arr : GenericArray<u32, $u> = GenericArray::new();
            b.iter(|| black_box(arr).map(|x| x + 1));
        }
        
        #[bench]
        fn $h(b: &mut Bencher) {
            let mut arr : ArrayVec<[u32; $n]> = ArrayVec::new();
            for _ in 0..32 { arr.push(0u32); }
            b.iter(|| {
                black_box(arr.iter()).map(|x| x + 1).collect::<ArrayVec<[u32; $n]>>();
            });
        }
        
        #[bench]
        fn $i(b: &mut Bencher) {
            let arr = [0u32; $n];
            b.iter(|| {
                black_box(arr).iter().map(|x| x + 1).collect::<Vec<_>>()
            });
        }
    };
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use generic_array::*;
    use arraymap::ArrayMap;
    use arrayvec::ArrayVec;
    use typenum::consts::*;

    bench_map!( U0,  0, array_00, gena_u00, arvec_00, vec_00);
    bench_map!( U1,  1, array_01, gena_u01, arvec_01, vec_01);
    bench_map!( U2,  2, array_02, gena_u02, arvec_02, vec_02);
    bench_map!( U3,  3, array_03, gena_u03, arvec_03, vec_03);
    bench_map!( U4,  4, array_04, gena_u04, arvec_04, vec_04);
    bench_map!( U5,  5, array_05, gena_u05, arvec_05, vec_05);
    bench_map!( U6,  6, array_06, gena_u06, arvec_06, vec_06);
    bench_map!( U7,  7, array_07, gena_u07, arvec_07, vec_07);
    bench_map!( U8,  8, array_08, gena_u08, arvec_08, vec_08);
    bench_map!( U9,  9, array_09, gena_u09, arvec_09, vec_09);
    bench_map!(U10, 10, array_10, gena_u10, arvec_10, vec_10);
    bench_map!(U11, 11, array_11, gena_u11, arvec_11, vec_11);
    bench_map!(U12, 12, array_12, gena_u12, arvec_12, vec_12);
    bench_map!(U13, 13, array_13, gena_u13, arvec_13, vec_13);
    bench_map!(U14, 14, array_14, gena_u14, arvec_14, vec_14);
    bench_map!(U15, 15, array_15, gena_u15, arvec_15, vec_15);
    bench_map!(U16, 16, array_16, gena_u16, arvec_16, vec_16);
    bench_map!(U17, 17, array_17, gena_u17, arvec_17, vec_17);
    bench_map!(U18, 18, array_18, gena_u18, arvec_18, vec_18);
    bench_map!(U19, 19, array_19, gena_u19, arvec_19, vec_19);
    bench_map!(U20, 20, array_20, gena_u20, arvec_20, vec_20);
    bench_map!(U21, 21, array_21, gena_u21, arvec_21, vec_21);
    bench_map!(U22, 22, array_22, gena_u22, arvec_22, vec_22);
    bench_map!(U23, 23, array_23, gena_u23, arvec_23, vec_23);
    bench_map!(U24, 24, array_24, gena_u24, arvec_24, vec_24);
    bench_map!(U25, 25, array_25, gena_u25, arvec_25, vec_25);
    bench_map!(U26, 26, array_26, gena_u26, arvec_26, vec_26);
    bench_map!(U27, 27, array_27, gena_u27, arvec_27, vec_27);
    bench_map!(U28, 28, array_28, gena_u28, arvec_28, vec_28);
    bench_map!(U29, 29, array_29, gena_u29, arvec_29, vec_29);
    bench_map!(U30, 30, array_30, gena_u30, arvec_30, vec_30);
    bench_map!(U31, 31, array_31, gena_u31, arvec_31, vec_31);
    bench_map!(U32, 32, array_32, gena_u32, arvec_32, vec_32);
}
