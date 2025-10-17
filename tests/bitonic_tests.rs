use code_kata_bitonic_sequence_rust::get_bitonic_sequence_v2;

// validar com o grupo a ideia

fn is_strict_bitonic(seq: &Vec<i32>) -> bool {
    if seq.len() < 3 {
        return false;
    }

    let &maxv = seq.iter().max().unwrap();
    let &minv = seq.iter().min().unwrap();

    let peak_up_idx = seq.iter().position(|&v| v == maxv);
    let peak_down_idx = seq.iter().position(|&v| v == minv);

    if let Some(idx) = peak_up_idx {
        if idx != 0 && idx != seq.len() - 1 {
            for i in 0..idx {
                if !(seq[i] < seq[i + 1]) {
                    return false;
                }
            }
            for i in idx..(seq.len() - 1) {
                if !(seq[i] > seq[i + 1]) {
                    return false;
                }
            }
            return true;
        }
    }

    if let Some(idx) = peak_down_idx {
        if idx != 0 && idx != seq.len() - 1 {
            for i in 0..idx {
                if !(seq[i] > seq[i + 1]) {
                    return false;
                }
            }
            for i in idx..(seq.len() - 1) {
                if !(seq[i] < seq[i + 1]) {
                    return false;
                }
            }
            return true;
        }
    }

    false
}

#[test]
fn test_case1() {
    let seq = get_bitonic_sequence_v2(5, 3, 10);
    assert_eq!(seq, vec![9, 10, 9, 8, 7]);
    // assert_eq!(seq, vec![7, 8, 9, 10, 9]); verificar se vale o teste
}

#[test]
fn test_case2() {
    let seq = get_bitonic_sequence_v2(7, 2, 5);
    assert_eq!(seq, vec![2, 3, 4, 5, 4, 3, 2]);
}

#[test]
fn test_case3_impossible() {
    let seq = get_bitonic_sequence_v2(5, 7, 8);
    assert_eq!(seq, vec![-1]);
}

#[test]
fn test_case2_min_size() {
    let seq = get_bitonic_sequence_v2(1, 3, 10);
    assert_eq!(seq, vec![9]);
}

// estudar esse teste

// #[test]
// fn first_two_elements_are_r_minus_one_then_r_when_possible() {
//     let n = 6;
//     let l = 1;
//     let r = 10;
//     let seq = get_bitonic_sequence_v2(n, l, r);

//     assert_eq!(seq.len(), n as usize, "Tamanho deve ser n");
//     assert!(
//         seq[0] == r - 1,
//         "Pelo algoritmo descrito, primeiro elemento deve ser r-1 ({}), obteve {} na seq {:?}",
//         r - 1,
//         seq[0],
//         seq
//     );
//     assert!(
//         seq[1] == r,
//         "Pelo algoritmo descrito, segundo elemento deve ser r ({}), obteve {} na seq {:?}",
//         r,
//         seq[1],
//         seq
//     );

//     assert!(
//         is_strict_bitonic(&seq),
//         "Sequência não respeita propriedade bitônica: {:?}",
//         seq
//     );
// }
