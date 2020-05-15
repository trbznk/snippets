import pandas as pd
import matplotlib.pyplot as plt

cols = ["date", "time", "score"]
df = pd.read_table("scores.txt", header=None, sep=" ", names=cols)
df["datetime"] = df["date"]+" "+df["time"]
df["datetime"] = pd.to_datetime(df["datetime"])
df = df.set_index(df["datetime"])
df = df.drop(["date", "time", "datetime"], axis=1)

df.plot(lw=.5, style="black", title="Scores over time", legend=False)
plt.tick_params(
    axis='x',          # changes apply to the x-axis
    which='both',      # both major and minor ticks are affected
    bottom=False,      # ticks along the bottom edge are off
    top=False,         # ticks along the top edge are off
    labelbottom=False)

plt.savefig("scores.png")
