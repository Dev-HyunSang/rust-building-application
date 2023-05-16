fn main() {
    // 러스트 모든 변수는 불변 변수임. 변수 값을 할당 되었다면 절대로 변경이 불가능함.
    // 가변 변수로 바꾸기 위해서는 선언한 후 `mut`를 사용하면 됨.
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight / 9.81) * 3.711
}