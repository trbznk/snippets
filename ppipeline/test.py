import pandas as pd
import glob
import numpy as np
import sys

from datetime import datetime
from tqdm import tqdm
from src import PPipeline

arg = sys.argv[-1].lower()
dev = True if arg == "-d" else False

scores = []
for path in tqdm(glob.glob("benchmarks/*.pickle")):
    df = pd.read_pickle(path)
    ppl = PPipeline(df, verbose=0, dev=dev)
    score = ppl.run()
    scores.append(score)

score = np.array(scores).mean()
print(score)

if not dev:
    with open("scores.txt", "a") as f:
        ts = str(datetime.now())
        f.write(f"{ts} {score}\n")
