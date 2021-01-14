#r "nuget: Microsoft.ML"

open System
open System.IO

open Microsoft.ML
open Microsoft.ML.Data
open Microsoft.ML.Transforms

/// CLIMutable
/// We want a C#-style class implementation with
/// a default constructor and setter functions for
/// every property.

/// It holds one single MNIST digit image. 
/// It combines 784 pixel columns into a single vector value.
[<CLIMutable>]
type Digit = {
    [<LoadColumn(0)>] Number: float32
    [<LoadColumn(1, 784)>] [<VectorType(784)>] PixelValues: float32 []
}

/// Holds a single prediction. The model with generate 10 scores,
/// one for every possible digt value.
[<CLIMutable>]
type DigitPrediction =  {
    Score: float32 []
}

// You can download the mnist files from
// https://www.kaggle.com/oddrationale/mnist-in-csv/

let trainDataPath = "./mnist_train.csv"
let testDataPath  = "./mnist_test.csv"

let mlCtx = MLContext()

let trainData = mlCtx.Data.LoadFromTextFile<Digit>(trainDataPath, hasHeader = true, separatorChar = ',')
let testData  = mlCtx.Data.LoadFromTextFile<Digit>(testDataPath,  hasHeader = true, separatorChar = ',')

let pipeline =
    EstimatorChain()
        // step1: map the number column to a key value and store in the label column
        // we need this because we can only train a multiclass on keys.
        .Append(mlCtx.Transforms.Conversion.MapValueToKey("Label", "Number", keyOrdinality = ValueToKeyMappingEstimator.KeyOrdinality.ByValue))

        // step2: concatenate all feature columns
        // this is required because we can only train on a single input column.
        .Append(mlCtx.Transforms.Concatenate("Features", "PixelValues"))

        // step3: cache data to speed up training
        // optimization step that speeds up the learning algorithm.
        .AppendCacheCheckpoint(mlCtx)

        // step4: traint eh model with SDCA
        // a Sdca learner which will train the model to make accurate predictions.
        .Append(mlCtx.MulticlassClassification.Trainers.SdcaMaximumEntropy())

        // step5: map the label key value back to a number
        // converts the keys in the label column back to the original number values.
        // we need to show the numbers when making predictions.
        .Append(mlCtx.Transforms.Conversion.MapKeyToValue("Number", "Label"))

let model = trainData |> pipeline.Fit

let metrics = testData |> model.Transform |> mlCtx.MulticlassClassification.Evaluate

// show the evaluation metrics
printfn "Evaluation metrics"

// this is the average accuracy, the number of correct predictions
// divided by the total number of predictions for every digit in the dataset.
printfn "  MicroAccuracy:    %f" metrics.MicroAccuracy

// this is calculated by first calculating the average accuracy for each unique
// prediction value, and then taking the averages of thos averages.
printfn "  MacroAccuracy:    %f" metrics.MacroAccuracy

// this is a metric that expreses the size of the error in the predictions
// the model is making. A logloss of zero means every prediction is correct,
// and the loss values rises as the movel makes more and more mistakes.
printfn "  LogLoss:          %f" metrics.LogLoss

// this metric is also called Reduction in Information Gain (RIG).
// it expresses the probability that the model's predictions are 
// bettr than random chance.
printfn "  LogLossReduction: %f" metrics.LogLossReduction

// in an unbiased set each unique label value will appear roughly the same number
// of times, and the micro- and macro- accuracy values will be close together.
// if the values are far apart, this suggests that there is some kind of bias in
// the data tat we need to deal with.

let digits = mlCtx.Data.CreateEnumerable(testData, reuseRowObject = false) |> Array.ofSeq
let testDigits = [digits.[5]; digits.[16]; digits.[28]; digits.[63]; digits.[129]]

// create a prediction engine
let engine = mlCtx.Model.CreatePredictionEngine model

// show predictions
printfn "Model predictions:"
printf "  #\t\t"; [0..9] |> Seq.iter(fun i -> printf "%i\t\t" i); printfn ""
testDigits |> Seq.iter(
    fun digit ->
        printf "  %i\t" (int digit.Number)
        let p = engine.Predict digit
        p.Score |> Seq.iter (fun s -> printf "%f\t" s)
        printfn ""
)