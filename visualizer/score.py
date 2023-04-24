# %%
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# %%
before_df = pd.read_csv("out/local/stats.csv")
after_df = pd.read_csv("out/local/stats_after.csv")

# %%
sns.lineplot(data=before_df, x="round", y="score", label="bebore")
sns.lineplot(data=after_df, x="round", y="score", label="after")

# %%
sns.lineplot(data=before_df, x="time", y="score", label="bebore")
sns.lineplot(data=after_df, x="time", y="score", label="after")

# %%
