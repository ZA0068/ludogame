import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl
import numpy as np
import sys
import os

# Load the winrate data

filename = sys.argv[1]

winrate_data = pd.read_csv(f'./data/{filename}/{filename}_winrates.csv', index_col='tournament')
os.makedirs(f'./data/{filename}/images', exist_ok=True)

numeric_winrate_data = winrate_data.select_dtypes(include=[np.number])  # assuming you have numpy imported as np
# Compute the statistics
data_stats = pd.DataFrame(index=winrate_data.index)
data_stats['Max'] = numeric_winrate_data.max(axis=1)
data_stats['Upper Quantile'] = numeric_winrate_data.quantile(0.75, axis=1)
data_stats['Median'] = numeric_winrate_data.median(axis=1)
data_stats['Mean'] = numeric_winrate_data.mean(axis=1)
data_stats['Lower Quantile'] = numeric_winrate_data.quantile(0.25, axis=1)
data_stats['Min'] = numeric_winrate_data.min(axis=1)
data_stats['STD'] = numeric_winrate_data.std(axis=1)
# Load the parameter data
param_data = pd.read_csv(f'./data/{filename}/{filename}_params.csv')
total_generations = param_data.loc[0, 'Total Generations']
# Brightening the colors and increasing the line thickness
colors = {
    'Max': 'blue',
    'Upper Quantile': 'cyan',
    'Median': 'lime',
    'Mean': 'yellow',
    'Lower Quantile': 'darkorange',
    'Min': 'red',
    'STD': 'pink',
}

mpl.style.use('dark_background')

plt.figure(figsize=(16, 9))
for metric in data_stats.columns:
    plt.plot(data_stats.index, data_stats[metric], label=metric, color=colors[metric], linewidth=2.5)

plt.title(f"GA Players' Winrates for {filename} in {total_generations} generations", fontsize=16)
plt.xlabel("Generation", fontsize=14)
plt.ylabel("Winrate", fontsize=14)
plt.grid(True, linestyle='--', alpha=0.6)
plt.legend(fontsize=12, bbox_to_anchor=(1.05, 1), loc='upper left', borderaxespad=0.)

# Create a text box and place it outside the plot area
props = dict(boxstyle='round', facecolor='wheat', alpha=0.5)

textbox_text = '\n'.join((
    f'Total Generations = {total_generations}',
    f'Total Populations = {param_data.loc[0, "Total Populations"]}',
    f'Total Games = {param_data.loc[0, "Total Games"]}',
    f'Mutation rate = {param_data.loc[0, "Mutation Rate"]}',
    f'Crossover rate = {param_data.loc[0, "Crossover Rate"]}',
    f'Number of elites = {param_data.loc[0, "Number of Elites"]}',
))

plt.text(1.02, 0.5, textbox_text, transform=plt.gca().transAxes, fontsize=14, verticalalignment='top', bbox=props)



plt.savefig(f'./data/{filename}/images/{filename}_plot.png', dpi=300, bbox_inches='tight')
plt.tight_layout()
plt.show()
