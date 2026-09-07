// 1. Membaca dataset dari file CSV (2 baris, 2 kolom)
let X = read_csv(2, 2);

// 2. Inisialisasi bobot matriks
let W = [
    [0.5, 0.1],
    [-0.2, 0.8]
];

// 3. Matriks target
let Target = [
    [1.0, 0.0],
    [0.0, 1.0]
];

// 4. Forward Pass & Sigmoid Activation
let Z = X * W;
let Pred = sigmoid(Z);

// 5. Kalkulasi MSE Loss
let loss = mse_loss(Pred, Target);
print(loss);