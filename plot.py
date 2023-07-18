import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl

# Load the data
data = pd.read_csv("data/GA_data.csv", index_col='tournament')

# Compute the statistics
data_stats = pd.DataFrame(index=data.index)
data_stats['Max'] = data.max(axis=1)
data_stats['Upper Quantile'] = data.quantile(0.75, axis=1)
data_stats['Median'] = data.median(axis=1)
data_stats['Mean'] = data.mean(axis=1)
data_stats['Lower Quantile'] = data.quantile(0.25, axis=1)
data_stats['Min'] = data.min(axis=1)

# Brightening the colors and increasing the line thickness
colors = {
    'Max': 'blue',
    'Upper Quantile': 'cyan',
    'Median': 'lime',
    'Mean': 'yellow',
    'Lower Quantile': 'darkorange',
    'Min': 'red',
}

mpl.style.use('dark_background')

plt.figure(figsize=(12, 8))
for metric in data_stats.columns:
    plt.plot(data_stats.index, data_stats[metric], label=metric, color=colors[metric], linewidth=2.5)

plt.title("GA Players' Winrates for test result in 50 generations", fontsize=16)
plt.xlabel("Generation", fontsize=14)
plt.ylabel("Winrate", fontsize=14)
plt.grid(True, linestyle='--', alpha=0.6)
plt.legend(fontsize=12)
plt.show()