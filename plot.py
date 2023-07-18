import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl

# Load the winrate data
winrate_data = pd.read_csv("./data/GA test/GA test_winrates.csv", index_col='tournament')

# Compute the statistics
data_stats = pd.DataFrame(index=winrate_data.index)
data_stats['Max'] = winrate_data.max(axis=1)
data_stats['Upper Quantile'] = winrate_data.quantile(0.75, axis=1)
data_stats['Median'] = winrate_data.median(axis=1)
data_stats['Mean'] = winrate_data.mean(axis=1)
data_stats['Lower Quantile'] = winrate_data.quantile(0.25, axis=1)
data_stats['Min'] = winrate_data.min(axis=1)
data_stats['STD'] = winrate_data.std(axis=1)

# Load the parameter data
param_data = pd.read_csv("./data/GA test/GA test_params.csv")
total_generations = param_data.loc[0, 'Total Generations']
filename = 'GA test'

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

plt.savefig('./data/GA test/images/ga_data_plot2.png', dpi=300, bbox_inches='tight')
plt.show()
