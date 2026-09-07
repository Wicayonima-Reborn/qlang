let X = [
    [1.0, 2.0],
    [3.0, 4.0]
];

let W = [
    [0.5, 0.1],
    [-0.2, 0.8]
];

let Target = [
    [1.0, 0.0],
    [0.0, 1.0]
];

let Z = X * W;
let Pred = sigmoid(Z);

let loss = mse_loss(Pred, Target);
print(loss);