import tensorflow as tf
import os
import math

from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import mean_squared_error

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'


def naive(df):
    df = df.copy()
    df["naive"] = df["y"].shift(-1)
    df = df.dropna()
    return math.sqrt(mean_squared_error(df["naive"], df["y"]))


def gen_history(df, history=5):
    df = df.copy()
    for col in df.columns:
        for i in range(1, history):
            df[f"{col}-{i}"] = df["y"].shift(i)
    return df.dropna()


def gen_target(df):
    df = df.copy()
    df["y+1"] = df["y"].shift(-1)
    return df.dropna()


class PPipeline:
    def __init__(self, df, verbose=1, dev=False):
        self.df = df
        self.verbose = verbose
        self.dev = dev

    def run(self):
        benchmark_error = naive(self.df)
        self.df = gen_history(self.df)
        self.df = gen_target(self.df)

        y = self.df["y+1"]
        X = self.df.drop(["y+1"], axis=1)

        # TODO: Is the order of features important wegen timesteps for LSTM?

        X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.20, random_state=42, shuffle=False)

        scaler = StandardScaler()
        X_train = scaler.fit_transform(X_train)
        X_test = scaler.transform(X_test)

        # Parsing
        y_train = y_train.to_numpy()
        y_test = y_test.to_numpy()

        # The LSTM input layer must be 3D.
        # The meaning of the 3 input dimensions are: samples, time steps, and features.
        X_train = X_train.reshape(X_train.shape[0], X_test.shape[1], 1)
        X_test = X_test.reshape(X_test.shape[0], X_test.shape[1], 1)

        callback = tf.keras.callbacks.EarlyStopping(
            monitor="loss",
            patience=20,
            restore_best_weights=True
        )

        input_shape = (X_train.shape[1], X_train.shape[2])
        #neurons = (X_train.shape[1]+1)//2
        epochs = 10000
        neurons = 128
        model = tf.keras.models.Sequential()
        model.add(tf.keras.layers.LSTM(neurons, input_shape=input_shape, return_sequences=True))
        model.add(tf.keras.layers.Dropout(0.2))
        model.add(tf.keras.layers.LSTM(neurons, return_sequences=True))
        model.add(tf.keras.layers.Dropout(0.2))
        model.add(tf.keras.layers.LSTM(neurons))
        model.add(tf.keras.layers.Dropout(0.2))
        model.add(tf.keras.layers.Dense(1))

        model.compile(
            optimizer='adam',
            loss="mse",
            metrics=[tf.keras.metrics.RootMeanSquaredError()]
        )

        epochs = epochs if not self.dev else 1
        self.history_ = model.fit(
            X_train,
            y_train,
            validation_data=(X_test, y_test),
            batch_size=32,
            epochs=epochs,
            callbacks=[callback],
            verbose=self.verbose
        )

        y_pred = model.predict(X_test)
        y_pred = y_pred.reshape(len(y_pred))
        error = math.sqrt(mean_squared_error(y_test, y_pred))

        return benchmark_error/error
