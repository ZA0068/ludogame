import pandas as pd
import matplotlib.pyplot as plt

# Read data from CSV
df = pd.read_csv('generations.csv')

# Generate box plots
plt.figure(figsize=(10, 8))
plt.title('Box Plot of Generations')
df.boxplot()
plt.xlabel('Generation')
plt.ylabel('Value')
plt.show()

# Assume you have columns named 'best', 'worst', and 'average' in your DataFrame df
plt.figure(figsize=(10, 8))
plt.plot(df['best'], label='Best')
plt.plot(df['worst'], label='Worst')
plt.plot(df['average'], label='Average')
plt.xlabel('Generation')
plt.ylabel('Value')
plt.title('Line Plot of Best, Worst, and Average Values per Generation')
plt.legend()
plt.show()
