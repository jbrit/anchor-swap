pub fn get_optimal_input(_x: &mut [f64; 2], _y: &mut [f64; 2]) -> f64 {
    let _fees: [f64; 2] = [0.9975, 0.997];

    let a: f64 = _y.iter().product::<f64>() as f64 * _fees.iter().product::<f64>();
    let b: f64 = _x.iter().product::<f64>() as f64;
    let ff1: f64 = _fees[0] * _x.iter().skip(1).product::<f64>() as f64;
    let ff3: f64 =
        _y.iter().take(_y.len() - 1).product::<f64>() as f64 * _fees.iter().product::<f64>();
    let ff2 = (2.._x.len())
        .map(|j| {
            _x.iter().skip(j).product::<f64>() as f64
                * _y.iter().take(j - 1).product::<f64>() as f64
                * _fees.iter().take(j).product::<f64>()
        })
        .collect::<Vec<_>>();
    let sum: f64 = ff2.iter().sum();
    let c: f64 = ff1 + ff3 + sum;

    let _optimal_input: f64 = ((-b + ((a * b).powf(0.5))) / c) as f64;
    println!("optimal_input : {}", _optimal_input);
    return _optimal_input
}

pub fn get_amount_out(input_amount: f64, _x: &mut [f64; 2], _y: &mut [f64; 2]) -> f64 {
    let _fees: [f64; 2] = [0.9975, 0.997];
    let mut amount_in = input_amount;
    let mut amount_out = 1.0;
    let iter = _x.iter().zip(_y.iter()).zip(_fees.iter());
    for v in iter {
        amount_in = amount_in * v.1;
        let denominator = v.0.0 + amount_in;
        amount_out = v.0.1 * amount_in / denominator;
        amount_in = amount_out;
    }
    println!("amount_out : {}", amount_out);
    return amount_out
}