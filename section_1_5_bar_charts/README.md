# Section 1.5: Bar Charts in Python

This directory contains interactive Jupyter notebooks for learning about bar charts in data visualization using Python.

## Overview

Bar charts are a fundamental tool in data visualization used to depict data values for categorical variables. This section covers:

- **Basic bar charts** (vertical and horizontal)
- **Grouped bar charts** (comparing multiple groups)
- **Stacked bar charts** (showing composition)
- **Relative frequency charts** (showing percentages)

## Notebooks

### 1. Try 1.5.1: Bar Charts (`try_1_5_1_bar_charts.ipynb`)

Learn the fundamentals of creating bar charts using seaborn:
- Creating vertical bar charts with `sns.countplot(x=...)`
- Creating horizontal bar charts with `sns.countplot(y=...)`
- Customizing charts with titles, labels, and formatting
- Using `sns.barplot()` for displaying actual values

**Key Concepts:**
- Category axis vs. value axis
- Appropriate increments for readability
- When to use horizontal vs. vertical orientation
- Handling long category labels with rotation

### 2. Try 1.5.2: Grouped Bar Charts (`try_1_5_2_grouped_bar_charts.ipynb`)

Learn to compare multiple groups within categories:
- Using the `hue` parameter to create grouped charts
- Analyzing the famous Titanic dataset
- Creating charts for time-series comparisons (e.g., workforce data)
- Interpreting differences between groups

**Key Concepts:**
- Grouping data by categorical variables
- Using legends to distinguish groups
- Comparing trends across categories
- Both vertical and horizontal grouped charts

### 3. Try 1.5.3: Stacked Bar Charts (`try_1_5_3_stacked_bar_charts.ipynb`)

Learn to show composition and totals:
- Creating stacked bar charts with `plt.subplots()`
- Using the `bottom` parameter for stacking
- Creating relative frequency stacked charts (percentages)
- Displaying preterm birth data and state spending examples

**Key Concepts:**
- Showing both total and breakdown simultaneously
- Using `bottom` parameter for vertical stacking
- Using `left` parameter for horizontal stacking
- Adding data labels to bars
- Relative frequency visualization

## Getting Started

### Prerequisites

You need Python 3.6 or higher installed on your system.

### Installation

1. Install the required Python packages:

```bash
pip install -r requirements.txt
```

Or install individually:

```bash
pip install pandas numpy matplotlib seaborn jupyter
```

2. Launch Jupyter Notebook:

```bash
jupyter notebook
```

3. Open any of the `.ipynb` files in your browser and run the cells.

### Running the Notebooks

Each notebook is self-contained and includes:
- Import statements for required libraries
- Sample datasets (some use built-in seaborn datasets)
- Step-by-step code examples with explanations
- Visualization outputs
- Exercise sections for hands-on practice

**To run a notebook:**
1. Click "Restart & Run All" or use the double-right arrow button
2. Alternatively, run cells individually using Shift+Enter

## Learning Objectives

After completing these notebooks, you will be able to:

1. ✅ Determine appropriate uses for bar charts
2. ✅ Interpret bar charts and extract insights from data
3. ✅ Create vertical and horizontal bar charts
4. ✅ Create relative-frequency bar charts
5. ✅ Create grouped bar charts for comparing multiple groups
6. ✅ Create stacked bar charts to show composition
7. ✅ Customize charts with titles, labels, colors, and legends
8. ✅ Choose the right chart type for your data and audience

## Key Python Libraries Used

### Pandas
Data manipulation and analysis library
- Creating DataFrames
- Data cleaning and preparation

### Matplotlib
Core plotting library
- `plt.subplots()` for creating figures and axes
- `plt.bar()` and `plt.barh()` for bar charts
- Customization options for titles, labels, and formatting

### Seaborn
Statistical data visualization built on matplotlib
- `sns.countplot()` for counting categorical data
- `sns.barplot()` for plotting values
- Built-in datasets (e.g., Titanic)
- Beautiful default styling

## Examples and Datasets

The notebooks use several real-world examples:

1. **U.S. Private Employers (2017)** - Walmart, Amazon, Yum! Brands, Kroger
2. **Titanic Dataset** - Survival rates by class and gender
3. **U.S. Workforce (1970-2010)** - Gender composition over time
4. **Hospital PA/APN Utilization** - Healthcare provider usage by location
5. **Preterm Births (2014-2016)** - CDC data by mother's age group
6. **Massachusetts State Spending** - Healthcare vs. other spending trends

## Best Practices

When creating bar charts, remember to:

- ✅ Always include a descriptive title
- ✅ Label both axes clearly
- ✅ Use appropriate increments for the value axis
- ✅ Order categories logically (by value, alphabetically, or chronologically)
- ✅ Add grid lines for easier value estimation
- ✅ Use legends for grouped or stacked charts
- ✅ Consider horizontal orientation for long labels or many categories
- ✅ Use relative frequencies when comparing proportions
- ✅ Keep color choices accessible and meaningful

## Troubleshooting

### Common Issues

**Problem:** "No module named 'seaborn'"
**Solution:** Install seaborn: `pip install seaborn`

**Problem:** Plots don't display in Jupyter
**Solution:** Make sure `%matplotlib inline` is executed or use `plt.show()`

**Problem:** Titanic dataset not found
**Solution:** The dataset is downloaded automatically by seaborn. Ensure you have internet connectivity on first use.

## Additional Resources

- [Seaborn Documentation](https://seaborn.pydata.org/)
- [Matplotlib Documentation](https://matplotlib.org/)
- [Pandas Documentation](https://pandas.pydata.org/)
- [Python Graph Gallery - Bar Charts](https://python-graph-gallery.com/barplot/)

## Contributing

Feel free to suggest improvements or report issues with these notebooks.

## License

This educational content is part of the FlameLang project. See LICENSE file for details.
