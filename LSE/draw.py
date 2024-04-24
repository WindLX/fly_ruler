import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('output.csv')

headers = [
    'npos(ft)', 'epos(ft)',
    "altitude(ft)",
    "phi(degree)", "theta(degree)", "psi(degree)",
    "velocity(ft/s)",
    "alpha(degree)", "beta(degree)",
    "p(degree/s)", "q(degree/s)", "r(degree/s)",
    "nx(g)", "ny(g)", "nz(g)",
    "mach", "qbar(lb/ft ft)", "ps(lb/ft ft)"]

plt.figure(figsize=(16, 26))


def draw(index):
    plt.subplot(6, 3, index + 1)
    plt.plot(df['time(s)'], df[headers[index]])
    plt.title(f'Time vs {headers[index]}')
    plt.xlabel('Time (s)')
    plt.ylabel(headers[index])


for index, _h in enumerate(headers):
    draw(index)

plt.tight_layout()
plt.show()
