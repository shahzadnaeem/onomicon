use std::alloc::{Layout, LayoutError};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use itertools::iproduct;

pub fn run_hasher() {
    let mut hasher = DefaultHasher::new();

    let num = 1234;

    num.hash(&mut hasher);

    println!("Hash of num: {} is {:x}", num, hasher.finish());
}

pub fn run_sort() {
    let mut nums = vec![9, 3, 6, 1, 10];

    println!("Unsorted: {:?}", nums);

    nums.sort();

    println!("Sorted: {:?}", nums);

    assert_eq!(nums, vec![1, 3, 6, 9, 10]);
}

pub fn layout(
    sz: usize,
    al: usize,
    xt: usize,
    xt_sz: usize,
) -> Result<(Layout, usize), LayoutError> {
    let res = Layout::from_size_align(sz, al)?;

    let xtra = Layout::from_size_align(xt * xt_sz, xt_sz)?;

    let ext_res = res.extend(xtra)?;

    Ok((ext_res.0.pad_to_align(), ext_res.1))
}

pub fn run_layout() {
    // let aligns = vec![
    //     (7, 4, 14, 1),
    //     (7, 8, 14, 1),
    //     (7, 4, 14, 2),
    //     (7, 8, 14, 2),
    //     (7, 32, 14, 1),
    //     (7, 4, 14, 1),
    // ];

    // NOTE: Remake the above using iproduct! - consistently!

    let aligns = iproduct!(7..=7, 0..3, 14..=14, 0..2)
        .map(|(sz, al, xn, xsz)| (sz, 2 << al, xn, 1 << (xsz * 4)))
        .collect::<Vec<_>>();

    for a in aligns {
        println!(
            "\nLayout request: sz: {}, al: {}, xt: {}, xt_sz: {}",
            a.0, a.1, a.2, a.3
        );

        let l = layout(a.0, a.1, a.2, a.3);

        match l {
            Ok(mem) => {
                println!(
                    "Layout dimensions: sz: {}, al: {}, of: {}",
                    mem.0.size(),
                    mem.0.align(),
                    mem.1
                );
            }
            Err(err) => {
                println!("Layout error! {:?}", err);
            }
        }
    }
}

pub fn run_comps() {
    let lev1 = (0..8).map(|n| (7 - n)).collect::<Vec<_>>();

    println!("lev1: {:?}", lev1);

    // NOTE: Use of flat_map at all levels except where result is created - map used
    //       Clone of ranges that are 're-used'
    //

    let lev2 = (0..8)
        .flat_map(|n| (0..4).clone().map(move |w| (n, w)))
        .collect::<Vec<_>>();

    println!("lev2: {:?}", lev2);

    // NOTE: Much clearer via itertools::iproduct

    let lev3 = iproduct!((7..=7), (0..2), (7..9))
        .map(|(a, b, c)| (a, 2 << b, c))
        .collect::<Vec<_>>();

    println!("lev3: {:?}", lev3);
}

pub fn main() {
    // run_hasher();
    // run_sort();

    run_layout();
    // run_comps();
}
