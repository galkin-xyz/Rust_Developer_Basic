pub fn one_of_tuple(t: &mut (i32, i32), is_first: bool) -> &mut i32 {
    if is_first {
        &mut t.0
    } else {
        &mut t.1
    }
}

pub fn mut_ref_to_n_in_slice(slice: &mut [i32], n: usize) -> Option<&mut i32> {
    if n < slice.len() {
        Some(&mut slice[n])
    } else {
        None
    }
}

pub fn ref_to_n_from_end_in_slice(slice: &[i32], n: usize) -> Option<&i32> {
    if n < slice.len() {
        Some(&slice[slice.len() - 1 - n])
    } else {
        None
    }
}

pub fn div_slice_by_n(slice: &[i32], n: usize) -> Option<(&[i32], &[i32])> {
    if n < slice.len() {
        Some((&slice[..n], &slice[n..]))
    } else {
        None
    }
}

pub fn div_slice_to_4(slice: &[i32]) -> [&[i32]; 4] {
    let len = slice.len();
    let half1 = len / 2 + len % 2;
    let half2 = len / 2;
    let p1 = half1 / 2 + half1 % 2;
    let p2 = half2 / 2 + half2 % 2;
    let p3 = half1 / 2;
    //    let p4 = half2 / 2;

    [
        &slice[..p1],
        &slice[p1..p1 + p2],
        &slice[p1 + p2..p1 + p2 + p3],
        &slice[p1 + p2 + p3..],
    ]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_of_tuple_works() {
        let mut f: (i32, i32) = (12, -34);
        let mut s: (i32, i32) = (56, -78);

        let first: &mut i32 = one_of_tuple(&mut f, true);
        let second: &mut i32 = one_of_tuple(&mut s, false);

        assert_eq!(*first, 12);
        assert_eq!(*second, -78);

        *first = -120;
        *second = 780;

        assert_eq!(f.0, -120);
        assert_eq!(s.1, 780);
    }

    #[test]
    fn mut_ref_to_n_in_slice_works() {
        let mut array: [i32; 5] = [10; 5];
        let mut r1: &mut [i32] = &mut array[..];

        assert_eq!(mut_ref_to_n_in_slice(r1, 8), None);
        let elem: &mut i32 = mut_ref_to_n_in_slice(r1, 3).unwrap();
        assert_eq!(*elem, 10);
        *elem = 20;
        assert_eq!(array[3], 20);
        assert_eq!(array, [10, 10, 10, 20, 10]);

        r1 = &mut array[2..];
        assert_eq!(mut_ref_to_n_in_slice(r1, 4), None);
        let elem: &mut i32 = mut_ref_to_n_in_slice(r1, 0).unwrap();
        assert_eq!(*elem, 10);
        *elem = 30;
        assert_eq!(array[2], 30);
        assert_eq!(array, [10, 10, 30, 20, 10]);
    }

    #[test]
    fn ref_to_n_from_end_in_slice_works() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        let mut r1: &[i32] = &array[..];

        assert_eq!(ref_to_n_from_end_in_slice(r1, 8), None);
        assert_eq!(ref_to_n_from_end_in_slice(r1, 0), Some(&5));
        assert_eq!(ref_to_n_from_end_in_slice(r1, 2), Some(&3));
        assert_eq!(ref_to_n_from_end_in_slice(r1, 4), Some(&1));

        r1 = &array[1..4];
        assert_eq!(ref_to_n_from_end_in_slice(r1, 3), None);
        assert_eq!(ref_to_n_from_end_in_slice(r1, 0), Some(&4));
        assert_eq!(ref_to_n_from_end_in_slice(r1, 1), Some(&3));
        assert_eq!(ref_to_n_from_end_in_slice(r1, 2), Some(&2));
    }

    #[test]
    fn div_slice_by_n_works() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        let r1: &[i32] = &array[..];

        assert_eq!(div_slice_by_n(r1, 8), None);

        let res = div_slice_by_n(r1, 3).unwrap();
        assert_eq!(res.0, &[1, 2, 3]);
        assert_eq!(res.1, &[4, 5]);
    }

    #[test]
    fn div_slice_to_4_works() {
        let s: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8][..];

        assert_eq!(div_slice_to_4(&[]), [&[], &[], &[], &[]]);
        assert_eq!(
            div_slice_to_4(&s),
            [&[1, 2][..], &[3, 4][..], &[5, 6][..], &[7, 8][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..7]),
            [&[1, 2][..], &[3, 4][..], &[5, 6][..], &[7][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..6]),
            [&[1, 2][..], &[3, 4][..], &[5][..], &[6][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..5]),
            [&[1, 2][..], &[3][..], &[4][..], &[5][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..4]),
            [&[1][..], &[2][..], &[3][..], &[4][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..3]),
            [&[1][..], &[2][..], &[3][..], &[][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..2]),
            [&[1][..], &[2][..], &[][..], &[][..]]
        );
        assert_eq!(
            div_slice_to_4(&s[..1]),
            [&[1][..], &[][..], &[][..], &[][..]]
        );
    }
}
