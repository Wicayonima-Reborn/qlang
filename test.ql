let X = read_csv(2, 2);
let W = [[0.1, 0.2], [0.3, 0.4]];
let Target = [[1.0, 0.0], [0.0, 1.0]];

let Z = X @ W;
let Pred = sigmoid(Z);
let loss = mse_loss(Pred, Target);

train(loss, 0.5, 50);