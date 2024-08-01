fn main() {
    println!("Домашнее задание по теме: \"Типы данных. Переменные и функции\"");
    println!("double_int32(3600) = {}", double_int32(3600));
    println!("double_int64(-8530) = {}", double_int64(-8_530i32));
    println!("double_float32(3.1415) = {}", double_float32(3.1415));
    println!("double_float64(25.0) = {}", double_float64(25f32));
    println!(
        "int_plus_float_to_float(-20, -30.678) = {:.3}",
        int_plus_float_to_float(-20i32, -30.678)
    );
    println!(
        "ins_plus_float_to_int(20, 30.678) = {}",
        int_plus_float_to_int(20, 30.678)
    );
    println!("tuple_sum((50, 75)) = {}", tuple_sum((50, 75)));
    println!("array_sum([10, 20, -30]) = {}", array_sum([10, 20, -30]));
}

fn double_int32(int32: i32) -> i32 {
    int32.wrapping_mul(2)
}

fn double_int64(int32: i32) -> i64 {
    (int32 * 2).into()
}

fn double_float32(float32: f32) -> f32 {
    float32 * 2.0
}

fn double_float64(float32: f32) -> f64 {
    (float32 * 2.0).into()
}

fn int_plus_float_to_float(int32: i32, float32: f32) -> f64 {
    int32 as f64 + float32 as f64
}

fn int_plus_float_to_int(int32: i32, float32: f32) -> i64 {
    int32 as i64 + float32.round() as i64
}

fn tuple_sum(tuple: (i32, i32)) -> i32 {
    tuple.0.wrapping_add(tuple.1)
}

fn array_sum(array: [i32; 3]) -> i32 {
    array[0].wrapping_add(array[1]).wrapping_add(array[2])
}
