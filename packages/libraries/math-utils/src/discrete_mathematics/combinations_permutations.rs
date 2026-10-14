/// Calculates the number of combinations of `k` elements chosen from `n` elements
/// without replacement and without regard to order, also known as the binomial coefficient.
///
/// C(n, k) = P(n, k) / k! = n! / (k! * (n - k)!)
///
/// This function utilizes highly optimized bitwise arithmetic and modular inverses
/// for entries within the `u64` range to achieve O(1) performance where possible.
/// If an integer overflow would occur, it seamlessly falls back to an iterative
/// `f64` multiplicative formula approximation.
pub fn calculate_combinations(n: usize, k: usize) -> f64 {
    if let Some(result) = perm_comb_small(n as u64, k as u64, true) {
        return result as f64;
    }

    // C(n, k) = C(n, k-1) * (n-k+1) / k
    // see: https://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula
    let k = k.min(n - k);
    let n = n as f64;
    let mut result = 1.0;

    for i in 1..=k {
        let i_float = i as f64;
        result *= n + 1.0 - i_float;
        result /= i_float;
        result = result.round();
    }

    result
}

/// Calculates the number of ordered permutations of `k` elements chosen from `n`
/// elements without replacement.
///
/// P(n, k) = n! / (n - k)!
///
/// This function utilizes highly optimized bitwise arithmetic and modular inverses
/// for entries within the `u64` range to achieve O(1) performance where possible.
/// If an integer overflow would occur, it seamlessly falls back to an iterative
/// `f64` falling factorial loop approximation.
pub fn calculate_permutations(n: usize, k: usize) -> f64 {
    if let Some(result) = perm_comb_small(n as u64, k as u64, false) {
        return result as f64;
    }

    // P(n, k) = P(n, k-1) * (n-k+1)
    let n = n as f64;
    let mut result = 1.0;

    for i in 1..=k {
        result *= n + 1.0 - i as f64;
        result = result.round();
    }

    result
}

/*
    The following code is a portation of:
    Code: https://github.com/python/cpython/blob/main/Modules/mathintegermodule.c
*/

/// Least significant 64 bits of the odd part of factorial(n), for n in range(128).
const REDUCED_FACTORIAL_ODD_PART: [u64; 128] = [
    0x0000000000000001,
    0x0000000000000001,
    0x0000000000000001,
    0x0000000000000003,
    0x0000000000000003,
    0x000000000000000f,
    0x000000000000002d,
    0x000000000000013b,
    0x000000000000013b,
    0x0000000000000b13,
    0x000000000000375f,
    0x0000000000026115,
    0x000000000007233f,
    0x00000000005cca33,
    0x0000000002898765,
    0x00000000260eeeeb,
    0x00000000260eeeeb,
    0x0000000286fddd9b,
    0x00000016beecca73,
    0x000001b02b930689,
    0x00000870d9df20ad,
    0x0000b141df4dae31,
    0x00079dd498567c1b,
    0x00af2e19afc5266d,
    0x020d8a4d0f4f7347,
    0x335281867ec241ef,
    0x9b3093d46fdd5923,
    0x5e1f9767cc5866b1,
    0x92dd23d6966aced7,
    0xa30d0f4f0a196e5b,
    0x8dc3e5a1977d7755,
    0x2ab8ce915831734b,
    0x2ab8ce915831734b,
    0x81d2a0bc5e5fdcab,
    0x9efcac82445da75b,
    0xbc8b95cf58cde171,
    0xa0e8444a1f3cecf9,
    0x4191deb683ce3ffd,
    0xddd3878bc84ebfc7,
    0xcb39a64b83ff3751,
    0xf8203f7993fc1495,
    0xbd2a2a78b35f4bdd,
    0x84757be6b6d13921,
    0x3fbbcfc0b524988b,
    0xbd11ed47c8928df9,
    0x3c26b59e41c2f4c5,
    0x677a5137e883fdb3,
    0xff74e943b03b93dd,
    0xfe5ebbcb10b2bb97,
    0xb021f1de3235e7e7,
    0x33509eb2e743a58f,
    0x390f9da41279fb7d,
    0xe5cb0154f031c559,
    0x93074695ba4ddb6d,
    0x81c471caa636247f,
    0xe1347289b5a1d749,
    0x286f21c3f76ce2ff,
    0x00be84a2173e8ac7,
    0x1595065ca215b88b,
    0xf95877595b018809,
    0x9c2efe3c5516f887,
    0x373294604679382b,
    0xaf1ff7a888adcd35,
    0x18ddf279a2c5800b,
    0x18ddf279a2c5800b,
    0x505a90e2542582cb,
    0x5bacad2cd8d5dc2b,
    0xfe3152bcbff89f41,
    0xe1467e88bf829351,
    0xb8001adb9e31b4d5,
    0x2803ac06a0cbb91f,
    0x1904b5d698805799,
    0xe12a648b5c831461,
    0x3516abbd6160cfa9,
    0xac46d25f12fe036d,
    0x78bfa1da906b00ef,
    0xf6390338b7f111bd,
    0x0f25f80f538255d9,
    0x4ec8ca55b8db140f,
    0x4ff670740b9b30a1,
    0x8fd032443a07f325,
    0x80dfe7965c83eeb5,
    0xa3dc1714d1213afd,
    0x205b7bbfcdc62007,
    0xa78126bbe140a093,
    0x9de1dc61ca7550cf,
    0x84f0046d01b492c5,
    0x2d91810b945de0f3,
    0xf5408b7f6008aa71,
    0x43707f4863034149,
    0xdac65fb9679279d5,
    0xc48406e7d1114eb7,
    0xa7dc9ed3c88e1271,
    0xfb25b2efdb9cb30d,
    0x1bebda0951c4df63,
    0x5c85e975580ee5bd,
    0x1591bc60082cb137,
    0x2c38606318ef25d7,
    0x76ca72f7c5c63e27,
    0xf04a75d17baa0915,
    0x77458175139ae30d,
    0x0e6c1330bc1b9421,
    0xdf87d2b5797e8293,
    0xefa5c703e1e68925,
    0x2b6b1b3278b4f6e1,
    0xceee27b382394249,
    0xd74e3829f5dab91d,
    0xfdb17989c26b5f1f,
    0xc1b7d18781530845,
    0x7b4436b2105a8561,
    0x7ba7c0418372a7d7,
    0x9dbc5c67feb6c639,
    0x502686d7f6ff6b8f,
    0x6101855406be7a1f,
    0x9956afb5806930e7,
    0xe1f0ee88af40f7c5,
    0x984b057bda5c1151,
    0x9a49819acc13ea05,
    0x8ef0dead0896ef27,
    0x71f7826efe292b21,
    0xad80a480e46986ef,
    0x01cdc0ebf5e0c6f7,
    0x6e06f839968f68db,
    0xdd5943ab56e76139,
    0xcdcf31bf8604c5e7,
    0x7e2b4a847054a1cb,
    0x0ca75697a4d3d0f5,
    0x4703f53ac514a98b,
];

/// Inverses of reduced_factorial_odd_part values modulo 2**64.
const INVERTED_FACTORIAL_ODD_PART: [u64; 128] = [
    0x0000000000000001,
    0x0000000000000001,
    0x0000000000000001,
    0xaaaaaaaaaaaaaaab,
    0xaaaaaaaaaaaaaaab,
    0xeeeeeeeeeeeeeeef,
    0x4fa4fa4fa4fa4fa5,
    0x2ff2ff2ff2ff2ff3,
    0x2ff2ff2ff2ff2ff3,
    0x938cc70553e3771b,
    0xb71c27cddd93e49f,
    0xb38e3229fcdee63d,
    0xe684bb63544a4cbf,
    0xc2f684917ca340fb,
    0xf747c9cba417526d,
    0xbb26eb51d7bd49c3,
    0xbb26eb51d7bd49c3,
    0xb0a7efb985294093,
    0xbe4b8c69f259eabb,
    0x6854d17ed6dc4fb9,
    0xe1aa904c915f4325,
    0x3b8206df131cead1,
    0x79c6009fea76fe13,
    0xd8c5d381633cd365,
    0x4841f12b21144677,
    0x4a91ff68200b0d0f,
    0x8f9513a58c4f9e8b,
    0x2b3e690621a42251,
    0x4f520f00e03c04e7,
    0x2edf84ee600211d3,
    0xadcaa2764aaacdfd,
    0x161f4f9033f4fe63,
    0x161f4f9033f4fe63,
    0xbada2932ea4d3e03,
    0xcec189f3efaa30d3,
    0xf7475bb68330bf91,
    0x37eb7bf7d5b01549,
    0x46b35660a4e91555,
    0xa567c12d81f151f7,
    0x4c724007bb2071b1,
    0x0f4a0cce58a016bd,
    0xfa21068e66106475,
    0x244ab72b5a318ae1,
    0x366ce67e080d0f23,
    0xd666fdae5dd2a449,
    0xd740ddd0acc06a0d,
    0xb050bbbb28e6f97b,
    0x70b003fe890a5c75,
    0xd03aabff83037427,
    0x13ec4ca72c783bd7,
    0x90282c06afdbd96f,
    0x4414ddb9db4a95d5,
    0xa2c68735ae6832e9,
    0xbf72d71455676665,
    0xa8469fab6b759b7f,
    0xc1e55b56e606caf9,
    0x40455630fc4a1cff,
    0x0120a7b0046d16f7,
    0xa7c3553b08faef23,
    0x9f0bfd1b08d48639,
    0xa433ffce9a304d37,
    0xa22ad1d53915c683,
    0xcb6cbc723ba5dd1d,
    0x547fb1b8ab9d0ba3,
    0x547fb1b8ab9d0ba3,
    0x8f15a826498852e3,
    0x32e1a03f38880283,
    0x3de4cce63283f0c1,
    0x5dfe6667e4da95b1,
    0xfda6eeeef479e47d,
    0xf14de991cc7882df,
    0xe68db79247630ca9,
    0xa7d6db8207ee8fa1,
    0x255e1f0fcf034499,
    0xc9a8990e43dd7e65,
    0x3279b6f289702e0f,
    0xe7b5905d9b71b195,
    0x03025ba41ff0da69,
    0xb7df3d6d3be55aef,
    0xf89b212ebff2b361,
    0xfe856d095996f0ad,
    0xd6e533e9fdf20f9d,
    0xf8c0e84a63da3255,
    0xa677876cd91b4db7,
    0x07ed4f97780d7d9b,
    0x90a8705f258db62f,
    0xa41bbb2be31b1c0d,
    0x6ec28690b038383b,
    0xdb860c3bb2edd691,
    0x0838286838a980f9,
    0x558417a74b36f77d,
    0x71779afc3646ef07,
    0x743cda377ccb6e91,
    0x7fdf9f3fe89153c5,
    0xdc97d25df49b9a4b,
    0x76321a778eb37d95,
    0x7cbb5e27da3bd487,
    0x9cff4ade1a009de7,
    0x70eb166d05c15197,
    0xdcf0460b71d5fe3d,
    0x5ac1ee5260b6a3c5,
    0xc922dedfdd78efe1,
    0xe5d381dc3b8eeb9b,
    0xd57e5347bafc6aad,
    0x86939040983acd21,
    0x395b9d69740a4ff9,
    0x1467299c8e43d135,
    0x5fe440fcad975cdf,
    0xcaa9a39794a6ca8d,
    0xf61dbd640868dea1,
    0xac09d98d74843be7,
    0x2b103b9e1a6b4809,
    0x2ab92d16960f536f,
    0x6653323d5e3681df,
    0xefd48c1c0624e2d7,
    0xa496fefe04816f0d,
    0x1754a7b07bbdd7b1,
    0x23353c829a3852cd,
    0xbf831261abd59097,
    0x57a8e656df0618e1,
    0x16e9206c3100680f,
    0xadad4c6ee921dac7,
    0x635f2b3860265353,
    0xdd6d0059f44b3d09,
    0xac4dd6b894447dd7,
    0x42ea183eeaa87be3,
    0x15612d1550ee5b5d,
    0x226fa19d656cb623,
];

/// Exponent of the largest power of 2 dividing factorial(n), for n in range(68).
const FACTORIAL_TRAILING_ZEROS: [u8; 128] = [
    0, 0, 1, 1, 3, 3, 4, 4, 7, 7, 8, 8, 10, 10, 11, 11, //  0-15
    15, 15, 16, 16, 18, 18, 19, 19, 22, 22, 23, 23, 25, 25, 26,
    26, // 16-31
    31, 31, 32, 32, 34, 34, 35, 35, 38, 38, 39, 39, 41, 41, 42,
    42, // 32-47
    46, 46, 47, 47, 49, 49, 50, 50, 53, 53, 54, 54, 56, 56, 57,
    57, // 48-63
    63, 63, 64, 64, 66, 66, 67, 67, 70, 70, 71, 71, 73, 73, 74,
    74, // 64-79
    78, 78, 79, 79, 81, 81, 82, 82, 85, 85, 86, 86, 88, 88, 89,
    89, // 80-95
    94, 94, 95, 95, 97, 97, 98, 98, 101, 101, 102, 102, 104, 104, 105,
    105, // 96-111
    109, 109, 110, 110, 112, 112, 113, 113, 116, 116, 117, 117, 119, 119, 120,
    120, // 112-127
];

/// Number of permutations and combinations.
///
/// P(n, k) = n! / (n-k)!
///
/// C(n, k) = P(n, k) / k!
///
/// Calculate C(n, k) for n in the 63-bit range.
///
/// # Source
/// CPython's comb_perm_small
///
/// code: https://github.com/python/cpython/blob/main/Modules/mathintegermodule.c
fn perm_comb_small(n: u64, k: u64, is_comb: bool) -> Option<u64> {
    if k == 0 {
        // assert(k != 0)
        return Some(1);
    }

    // For small enough n and k the result fits in the 64-bit range and can
    // be calculated without allocating intermediate BigInts-like objects.
    if is_comb {
        // Maps k to the maximal n so that 2*k-1 <= n <= 127 and C(n, k)
        // fits into a uint64_t.  Exclude k = 1, because the second fast
        // path is faster for this case.
        const FAST_COMB_LIMITS1: [u8; 35] = [
            0, 0, 127, 127, 127, 127, 127, 127, // 0-7
            127, 127, 127, 127, 127, 127, 127, 127, // 8-15
            116, 105, 97, 91, 86, 82, 78, 76, // 16-23
            74, 72, 71, 70, 69, 68, 68, 67, // 24-31
            67, 67, 67, // 32-34
        ];
        if k < FAST_COMB_LIMITS1.len() as u64
            && n <= FAST_COMB_LIMITS1[k as usize] as u64
        {
            let n = n as usize;
            let k = k as usize;

            /*
                comb(n, k) fits into a uint64_t. We compute it as

                    comb_odd_part << shift

                where 2**shift is the largest power of two dividing comb(n, k)
                and comb_odd_part is comb(n, k) >> shift. comb_odd_part can be
                calculated efficiently via arithmetic modulo 2**64, using three
                lookups and two uint64_t multiplications.
            */
            let comb_odd_part = REDUCED_FACTORIAL_ODD_PART[n]
                .wrapping_mul(INVERTED_FACTORIAL_ODD_PART[k])
                .wrapping_mul(INVERTED_FACTORIAL_ODD_PART[n - k]);
            let shift = FACTORIAL_TRAILING_ZEROS[n]
                - FACTORIAL_TRAILING_ZEROS[k]
                - FACTORIAL_TRAILING_ZEROS[n - k];
            return Some(comb_odd_part << shift);
        }

        // Maps k to the maximal n so that 2*k-1 <= n <= 127 and C(n, k)*k
        // fits into a long long (which is at least 64 bit).  Only contains
        // items larger than in fast_comb_limits1.
        const FAST_COMB_LIMITS2: [u64; 14] = [
            0,
            u64::MAX,
            4294967296,
            3329022,
            102570,
            13467,
            3612,
            1449, // 0-7
            746,
            453,
            308,
            227,
            178,
            147, // 8-13
        ];
        if k < FAST_COMB_LIMITS2.len() as u64
            && n <= FAST_COMB_LIMITS2[k as usize]
        {
            // C(n, k) = C(n, k-1) * (n-k+1) / k
            let mut result = n;
            let mut n = n;
            let mut i = 1;
            while i < k {
                n -= 1;
                result *= n;

                i += 1;
                result /= i;
            }

            return Some(result);
        }
    } else {
        // Maps k to the maximal n so that k <= n and P(n, k)
        // fits into a long long (which is at least 64 bit).
        const FAST_PERM_LIMITS: [u64; 21] = [
            0,
            u64::MAX,
            4294967296,
            2642246,
            65537,
            7133,
            1627,
            568, // 0-7
            259,
            142,
            88,
            61,
            45,
            36,
            30,
            26, // 8-15
            24,
            22,
            21,
            20,
            20, // 16-20
        ];
        if k < FAST_PERM_LIMITS.len() as u64
            && n <= FAST_PERM_LIMITS[k as usize]
        {
            let k = k as usize;

            if n <= 127 {
                let n = n as usize;

                // P(n, k) fits into a uint64_t.
                let perm_odd_part = REDUCED_FACTORIAL_ODD_PART[n]
                    .wrapping_mul(INVERTED_FACTORIAL_ODD_PART[n - k]);
                let shift = FACTORIAL_TRAILING_ZEROS[n]
                    - FACTORIAL_TRAILING_ZEROS[n - k];
                return Some(perm_odd_part << shift);
            }

            // P(n, k) = P(n, k-1) * (n-k+1)
            let mut result = n;
            let mut n = n;
            for _i in 1..k {
                n -= 1;
                result *= n;
            }
            return Some(result);
        }
    }

    /* For larger n use recursive formulas:
     *
     *   P(n, k) = P(n, j) * P(n-j, k-j)
     *   C(n, k) = C(n, j) * C(n-j, k-j) // C(k, j)
     */
    let j = k / 2;

    let a = perm_comb_small(n, j, is_comb)?;
    let b = perm_comb_small(n - j, k - j, is_comb)?;

    let mut result = a.checked_mul(b)?;
    if is_comb {
        // && a != NULL
        let divisor = perm_comb_small(k, j, true)?;
        result = result.checked_div(divisor)?; // floor div
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_combinations_k_zero() {
        assert_eq!(calculate_combinations(5, 0), 1.0);
    }

    #[test]
    fn calculate_combinations_k_one() {
        assert_eq!(calculate_combinations(5, 1), 5.0);
    }

    #[test]
    fn calculate_combinations_seven_choose_four() {
        assert_eq!(calculate_combinations(7, 4), 35.0);
    }

    #[test]
    fn calculate_combinations_k_equals_n() {
        assert_eq!(calculate_combinations(10, 10), 1.0);
    }

    #[test]
    fn calculate_combinations_symmetric() {
        assert_eq!(calculate_combinations(10, 3), 120.0);
        assert_eq!(calculate_combinations(10, 7), 120.0);
    }

    #[test]
    fn calculate_combinations_larger_value() {
        assert_eq!(calculate_combinations(20, 10), 184756.0);
    }

    #[test]
    fn calculate_combinations_huge_value() {
        assert_eq!(calculate_combinations(145, 23), 3.1524666500557704e26);
    }

    #[test]
    fn calculate_permutations_k_zero() {
        assert_eq!(calculate_permutations(5, 0), 1.0);
    }

    #[test]
    fn calculate_permutations_k_one() {
        assert_eq!(calculate_permutations(5, 1), 5.0);
    }

    #[test]
    fn calculate_permutations_five_choose_two() {
        assert_eq!(calculate_permutations(5, 2), 20.0);
    }

    #[test]
    fn calculate_permutations_seven_choose_four() {
        assert_eq!(calculate_permutations(7, 4), 840.0);
    }

    #[test]
    fn calculate_permutations_k_equals_n() {
        assert_eq!(calculate_permutations(5, 5), 120.0);
    }

    #[test]
    fn calculate_permutations_larger_value() {
        assert_eq!(calculate_permutations(10, 5), 30240.0);
    }

    #[test]
    fn calculate_permutations_huge_value() {
        assert_eq!(calculate_permutations(145, 23), 8.14976206060184e48);
    }
}
