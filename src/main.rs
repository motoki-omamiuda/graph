// wgpu から ndarray に変更
use burn::backend::NdArray; 
use burn::tensor::Tensor;

fn main() {
    // バックエンドを NdArray に変更（f32は浮動小数点の精度）
    type MyBackend = NdArray<f32>;
    let device = Default::default();

    // テンソルの作成
    let tensor_a = Tensor::<MyBackend, 2>::from_data(
        [[1.0, 2.0, 3.0], 
         [4.0, 5.0, 6.0]], 
        &device
    );

    let tensor_b = Tensor::<MyBackend, 2>::from_data(
        [[7.0, 8.0], 
         [9.0, 10.0], 
         [11.0, 12.0]], 
        &device
    );

    println!("--- Tensor A --- \n{}", tensor_a);
    println!("--- Tensor B --- \n{}", tensor_b);

    // 行列の掛け算
    let result = tensor_a.matmul(tensor_b);

    println!("--- Result (A * B) --- \n{}", result);
}