use utils::*;
use matrix::Matrix;
use std::cmp::max;
use calc;
use std::str::FromStr;
use std::iter::Iterator;
use std::collections::{HashMap};
use fnv::FnvHashMap;

pub fn p011() -> Option<u64> {
    sub_execute(11, "a", p011a);
    sub_execute(11, "b", p011b);
    sub_execute(11, "c", p011c);
    None
}

/// Initial version. Multiple passes over the matrix.
fn p011a() -> Option<u64> {
    let input = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

    let m = Matrix::<u64>::from_str(input).unwrap();
    assert_eq!(m.rows(), 20);
    assert_eq!(m.cols(), 20);

    // Exclusive upper-bounds for loop limits.
    let horz_column_ub = m.cols() - 3;
    let vert_row_ub = m.rows() - 3;

    let mut answer = 0;

    // Generate 4-tuples horizontally.
    for col in 0..horz_column_ub {
        for row in 0..m.rows() {
            let product = m[row][col] * m[row][col + 1] * m[row][col + 2] * m[row][col + 3];
            answer = max(answer, product);
        }
    }

    // Generate 4-tuples vertically.
    for col in 0..m.cols() {
        for row in 0..vert_row_ub {
            let product = m[row][col] * m[row + 1][col] * m[row + 2][col] * m[row + 3][col];
            answer = max(answer, product);
        }
    }

    // Generate 4-tuples diagonally to the lower right.
    for col in 0..horz_column_ub {
        for row in 0..vert_row_ub {
            let product = m[row][col] *
                               m[row + 1][col + 1] *
                               m[row + 2][col + 2] *
                               m[row + 3][col + 3];

            answer = max(answer, product);
        }
    }

    // Generate 4-tuples diagonally to the lower left.
    for col in 3..m.cols() {
        for row in 0..vert_row_ub {
            let product = m[row][col] *
                m[row + 1][col - 1] *
                m[row + 2][col - 2] *
                m[row + 3][col - 3];

            answer = max(answer, product);
        }
    }

    assert_eq!(answer, 70_600_674);
    Some(answer)
}

/// Second version: one pass, using zero-padding to avoid indexing out of bounds.
fn p011b() -> Option<u64> {
    let input = "00 00 00 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 00 00 00
00 00 00 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 00 00 00
00 00 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 00 00 00
00 00 00 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 00 00 00
00 00 00 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 00 00 00
00 00 00 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 00 00 00
00 00 00 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 00 00 00
00 00 00 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 00 00 00
00 00 00 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 00 00 00
00 00 00 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 00 00 00
00 00 00 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 00 00 00
00 00 00 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 00 00 00
00 00 00 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 00 00 00
00 00 00 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 00 00 00
00 00 00 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 00 00 00
00 00 00 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 00 00 00
00 00 00 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 00 00 00
00 00 00 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 00 00 00
00 00 00 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 00 00 00
00 00 00 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    let m = Matrix::<u64>::from_str(input).unwrap();
    assert_eq!(m.rows(), 23);
    assert_eq!(m.cols(), 26);

    let mut answer = 0;

    for r in 0..20 {
        for c in 3..23 {
            // Generate 4-tuples horizontally.
            let product = m[r][c] * m[r][c + 1] * m[r][c + 2] * m[r][c + 3];
            answer = max(answer, product);
            // Generate 4-tuples vertically.
            let product = m[r][c] * m[r + 1][c] * m[r + 2][c] * m[r + 3][c];
            answer = max(answer, product);
            // Generate 4-tuples diagonally to the lower right.
            let product = m[r][c] * m[r + 1][c + 1] * m[r + 2][c + 2] * m[r + 3][c + 3];
            answer = max(answer, product);
            // Generate 4-tuples diagonally to the lower left.
            let product = m[r][c] * m[r + 1][c - 1] * m[r + 2][c - 2] * m[r + 3][c - 3];
            answer = max(answer, product);
        }
    }

    assert_eq!(answer, 70_600_674);
    Some(answer)
}

/// Third version: one pass, using ifs to avoid indexing out of bounds.
/// This is the fastest, by about a factor of 2.
fn p011c() -> Option<u64> {
    let input = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

    let m = Matrix::<u64>::from_str(input).unwrap();
    assert_eq!(m.rows(), 20);
    assert_eq!(m.cols(), 20);
    let mut answer = 0;

    // Off-by-one check:
    // If cols() is 20, then valid indexes are 0 to 19, inclusive. col_upper will be 17.
    // The loops below use <, so the last index used is 16, from which we add on a max of 3,
    // giving 17, 18 and 19, which does not exceed the limit.
    let col_upper = m.cols() - 3;
    let row_upper = m.rows() - 3;

    // Originally all 4 blocks were at the same level, but then I noticed that some conditions
    // (the outer ones such as c < col_upper) were common. This allowed a further optimization
    // of creating sub-conditions.
    for r in 0..m.rows() {
        for c in 0..m.cols() {
            if c < col_upper {
                // Generate 4-tuples horizontally.
                let product = m[r][c] * m[r][c + 1] * m[r][c + 2] * m[r][c + 3];
                answer = max(answer, product);

                // Generate 4-tuples diagonally to the lower right.
                if r < row_upper {
                    let product = m[r][c] * m[r + 1][c + 1] * m[r + 2][c + 2] * m[r + 3][c + 3];
                    answer = max(answer, product);
                }
            }

            if r < row_upper {
                // Generate 4-tuples vertically.
                let product = m[r][c] * m[r + 1][c] * m[r + 2][c] * m[r + 3][c];
                answer = max(answer, product);

                // Generate 4-tuples diagonally to the lower left.
                if c >= 3 {
                    let product = m[r][c] * m[r + 1][c - 1] * m[r + 2][c - 2] * m[r + 3][c - 3];
                    answer = max(answer, product);
                }
            }
        }
    }

    assert_eq!(answer, 70_600_674);
    Some(answer)
}

pub fn p012() -> Option<u64> {
    let mut answer = 0;

    for n in 5..100_000_000 {
        let tri = calc::triangle(n);
        let divisors = calc::num_divisors(tri);
        if divisors > 500 {
            answer = tri;
            break;
        }

//        if divisors.len() > max_divisors {
//            max_divisors = divisors.len();
//            answer = tri;
//        }
//
//        if max_divisors > 500 {
//            break;
//        }
//
//        if n % 100 == 0 {
//            println!("n = {}, tri = {}, max divisors = {}", n, tri, max_divisors);
//        }
    }

    assert_eq!(answer, 76_576_500);
    Some(answer)
}

pub fn p013() -> Option<u64> {
    let input = "37107287533902102798797998220837590246510135740250
46376937677490009712648124896970078050417018260538
74324986199524741059474233309513058123726617309629
91942213363574161572522430563301811072406154908250
23067588207539346171171980310421047513778063246676
89261670696623633820136378418383684178734361726757
28112879812849979408065481931592621691275889832738
44274228917432520321923589422876796487670272189318
47451445736001306439091167216856844588711603153276
70386486105843025439939619828917593665686757934951
62176457141856560629502157223196586755079324193331
64906352462741904929101432445813822663347944758178
92575867718337217661963751590579239728245598838407
58203565325359399008402633568948830189458628227828
80181199384826282014278194139940567587151170094390
35398664372827112653829987240784473053190104293586
86515506006295864861532075273371959191420517255829
71693888707715466499115593487603532921714970056938
54370070576826684624621495650076471787294438377604
53282654108756828443191190634694037855217779295145
36123272525000296071075082563815656710885258350721
45876576172410976447339110607218265236877223636045
17423706905851860660448207621209813287860733969412
81142660418086830619328460811191061556940512689692
51934325451728388641918047049293215058642563049483
62467221648435076201727918039944693004732956340691
15732444386908125794514089057706229429197107928209
55037687525678773091862540744969844508330393682126
18336384825330154686196124348767681297534375946515
80386287592878490201521685554828717201219257766954
78182833757993103614740356856449095527097864797581
16726320100436897842553539920931837441497806860984
48403098129077791799088218795327364475675590848030
87086987551392711854517078544161852424320693150332
59959406895756536782107074926966537676326235447210
69793950679652694742597709739166693763042633987085
41052684708299085211399427365734116182760315001271
65378607361501080857009149939512557028198746004375
35829035317434717326932123578154982629742552737307
94953759765105305946966067683156574377167401875275
88902802571733229619176668713819931811048770190271
25267680276078003013678680992525463401061632866526
36270218540497705585629946580636237993140746255962
24074486908231174977792365466257246923322810917141
91430288197103288597806669760892938638285025333403
34413065578016127815921815005561868836468420090470
23053081172816430487623791969842487255036638784583
11487696932154902810424020138335124462181441773470
63783299490636259666498587618221225225512486764533
67720186971698544312419572409913959008952310058822
95548255300263520781532296796249481641953868218774
76085327132285723110424803456124867697064507995236
37774242535411291684276865538926205024910326572967
23701913275725675285653248258265463092207058596522
29798860272258331913126375147341994889534765745501
18495701454879288984856827726077713721403798879715
38298203783031473527721580348144513491373226651381
34829543829199918180278916522431027392251122869539
40957953066405232632538044100059654939159879593635
29746152185502371307642255121183693803580388584903
41698116222072977186158236678424689157993532961922
62467957194401269043877107275048102390895523597457
23189706772547915061505504953922979530901129967519
86188088225875314529584099251203829009407770775672
11306739708304724483816533873502340845647058077308
82959174767140363198008187129011875491310547126581
97623331044818386269515456334926366572897563400500
42846280183517070527831839425882145521227251250327
55121603546981200581762165212827652751691296897789
32238195734329339946437501907836945765883352399886
75506164965184775180738168837861091527357929701337
62177842752192623401942399639168044983993173312731
32924185707147349566916674687634660915035914677504
99518671430235219628894890102423325116913619626622
73267460800591547471830798392868535206946944540724
76841822524674417161514036427982273348055556214818
97142617910342598647204516893989422179826088076852
87783646182799346313767754307809363333018982642090
10848802521674670883215120185883543223812876952786
71329612474782464538636993009049310363619763878039
62184073572399794223406235393808339651327408011116
66627891981488087797941876876144230030984490851411
60661826293682836764744779239180335110989069790714
85786944089552990653640447425576083659976645795096
66024396409905389607120198219976047599490197230297
64913982680032973156037120041377903785566085089252
16730939319872750275468906903707539413042652315011
94809377245048795150954100921645863754710598436791
78639167021187492431995700641917969777599028300699
15368713711936614952811305876380278410754449733078
40789923115535562561142322423255033685442488917353
44889911501440648020369068063960672322193204149535
41503128880339536053299340368006977710650566631954
81234880673210146739058568557934581403627822703280
82616570773948327592232845941706525094512325230608
22918802058777319719839450180888072429661980811197
77158542502016545090413245809786882778948721859617
72107838435069186155435662884062257473692284509516
20849603980134001723930671666823555245252804609722
53503534226472524250874054075591789781264330331690";

    // Convert to a sequence of int vectors. Left-pad the individual vectors with some
    // zeros to store carries. They help keep the code simple when we get to the last column.
    let mut input = input.lines()
        .map(|s| {
            let mut v = vec![0; 2];
            v.extend(s.chars().map(|c| c.to_digit(10).unwrap()));
            v
        })
       .collect::<Vec<_>>();

    let column_len = input[0].len();

    // This will hold the running sum.
    let mut answer = vec![0; column_len];

    // The max possibly sum of a column is 102 * 9 = 918, which means we need somewhere to
    // store up to two carry digits.
    input.push(answer.clone());
    input.push(answer.clone());

    let input_len = input.len();
    let carry1 = input_len - 1;
    let carry2 = input_len - 2;

    for col in (0..column_len).rev() {
        let mut column_sum = 0;
        for row in 0..input_len {
            column_sum += input[row][col];
        }

        let mut d = column_sum;
        let digit1 = d / 100;
        let d = d - digit1 * 100;
        let digit2 = d / 10;
        let d = d - digit2 * 10;
        let digit3 = d;

//        println!("Sum of column {} = {}, d1={}, d2={}, d3={}", col, column_sum, digit1, digit2, digit3);

        // column_sum has up to 3 digits. Store the rightmost (digit3) in the answer, and the
        // other two in available slots in the two carry terms.
        answer[col] = digit3;
        if digit2 > 0 {
            if input[carry1][col - 1] == 0 {
                input[carry1][col - 1] = digit2;
            } else {
                assert_eq!(0, input[carry2][col - 1], "Should not have carried into this digit");
                input[carry2][col - 1] = digit2;
            }
        }

        if digit1 > 0 {
            if input[carry1][col - 2] == 0 {
                input[carry1][col - 2] = digit1;
            } else {
                assert_eq!(0, input[carry2][col - 2], "Should not have carried into this digit");
                input[carry2][col - 2] = digit2;
            }
        }
    }

    let answer = calc::vec_to_num(&answer.into_iter()
        .skip_while(|&n| n == 0)
        .take(10)
        .map(u64::from)
        .collect::<Vec<_>>());

    assert_eq!(answer, 5_537_376_230);
    Some(answer)
}

pub fn p014() -> Option<u64> {
    sub_execute(14, "a (brute force)  ", p014a);
    sub_execute(14, "b (hashmap cache)", p014b);
    sub_execute(14, "c (struct cache) ", p014c);
    sub_execute(14, "d (FnvHashMap)   ", p014d);
    None
}

pub fn p014a() -> Option<u64> {
    let mut answer_len = 0;
    let mut answer_n = 0;

    for n in 2..1_000_000 {
        let clen = calc::collatz_len_simple(n);

        if clen > answer_len {
            answer_len = clen;
            answer_n = n;
            //println!("answer_n = {}, answer_len = {}", answer_n, answer_len);
        }
    }

    assert_eq!(answer_n, 837_799);
    Some(answer_n as u64)
}

pub fn p014b() -> Option<u64> {
    let mut known_collatzes = HashMap::<u32, u32>::new();
    known_collatzes.insert(1, 1);

    let mut answer_len = 0;
    let mut answer_n = 0;

    for n in 2..1_000_000 {
        let clen = calc::collatz_len(n, &mut known_collatzes);

        if clen > answer_len {
            answer_len = clen;
            answer_n = n;
            //println!("answer_n = {}, answer_len = {}", answer_n, answer_len);
        }
    }

    assert_eq!(answer_n, 837_799);
    Some(u64::from(answer_n))
}

pub fn p014c() -> Option<u64> {
    let mut known_collatzes = calc::KnownCollatzes::new();

    let mut answer_len = 0;
    let mut answer_n = 0;

    for n in 2..1_000_000 {
        let clen = calc::collatz_len2(n, &mut known_collatzes);

        if clen > answer_len {
            answer_len = clen;
            answer_n = n;
            //println!("answer_n = {}, answer_len = {}", answer_n, answer_len);
        }
    }

    assert_eq!(answer_n, 837_799);
    Some(answer_n as u64)
}

pub fn p014d() -> Option<u64> {
    let mut known_collatzes = FnvHashMap::default();
    known_collatzes.insert(1, 1);

    let mut answer_len = 0;
    let mut answer_n = 0;

    for n in 2..1_000_000 {
        let clen = calc::collatz_len3(n, &mut known_collatzes);

        if clen > answer_len {
            answer_len = clen;
            answer_n = n;
            //println!("answer_n = {}, answer_len = {}", answer_n, answer_len);
        }
    }

    assert_eq!(answer_n, 837_799);
    Some(answer_n as u64)
}