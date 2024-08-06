pub const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..VEC3_LEN {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        assert_eq!(VEC3_LEN, 3);
    }

    #[test]
    fn default_val() {
        for elem in default_vec3() {
            assert_eq!(elem, 0);
        }
    }

    #[test]
    fn vector_sum() {
        let a: Vec3 = [-10, 0, 20];
        let b: Vec3 = [5, 25, -35];
        let vec_sum: Vec3 = vec3_vector_sum(a, b);

        assert_eq!(vec_sum[0], -5);
        assert_eq!(vec_sum[1], 25);
        assert_eq!(vec_sum[2], -15);
    }

    #[test]
    fn scalar_sum() {
        let a: Vec3 = [2, 7, -4];
        let b: Vec3 = [-8, 6, 0];

        assert_eq!(vec3_scalar_sum(a, b), 3);
    }
}
