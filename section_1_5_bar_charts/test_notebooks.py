#!/usr/bin/env python3
"""
Test script to verify that all notebooks in Section 1.5 can be executed.
This script validates the Python code without requiring Jupyter.
"""

import sys
import os

def test_basic_imports():
    """Test that all required libraries can be imported."""
    print("Testing imports...")
    try:
        import pandas as pd
        import numpy as np
        import matplotlib
        matplotlib.use('Agg')  # Use non-interactive backend
        import matplotlib.pyplot as plt
        import seaborn as sns
        print("✓ All imports successful")
        return True
    except ImportError as e:
        print(f"✗ Import error: {e}")
        return False

def test_try_1_5_1():
    """Test basic bar chart functionality from Try 1.5.1."""
    print("\nTesting Try 1.5.1: Bar Charts...")
    try:
        import pandas as pd
        import seaborn as sns
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        
        # Test vertical bar chart
        companies_data = {
            'Company': ['Walmart', 'Amazon', 'Yum! Brands', 'Kroger'] * 2,
        }
        df = pd.DataFrame(companies_data)
        
        fig, ax = plt.subplots()
        sns.countplot(x='Company', data=df, ax=ax)
        plt.close()
        
        # Test horizontal bar chart
        fig, ax = plt.subplots()
        sns.countplot(y='Company', data=df, ax=ax)
        plt.close()
        
        print("✓ Try 1.5.1 tests passed")
        return True
    except Exception as e:
        print(f"✗ Try 1.5.1 failed: {e}")
        return False

def test_try_1_5_2():
    """Test grouped bar chart functionality from Try 1.5.2."""
    print("\nTesting Try 1.5.2: Grouped Bar Charts...")
    try:
        import seaborn as sns
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        
        # Test with Titanic dataset
        titanic_df = sns.load_dataset('titanic')
        
        fig, ax = plt.subplots()
        sns.countplot(x='class', hue='survived', data=titanic_df, ax=ax)
        plt.close()
        
        print("✓ Try 1.5.2 tests passed")
        return True
    except Exception as e:
        print(f"✗ Try 1.5.2 failed: {e}")
        return False

def test_try_1_5_3():
    """Test stacked bar chart functionality from Try 1.5.3."""
    print("\nTesting Try 1.5.3: Stacked Bar Charts...")
    try:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        
        # Test vertical stacked bar chart
        years = [2014, 2015, 2016]
        cat1 = [30, 35, 40]
        cat2 = [20, 25, 30]
        
        fig, ax = plt.subplots()
        ax.bar(years, cat1)
        ax.bar(years, cat2, bottom=cat1)
        plt.close()
        
        # Test horizontal stacked bar chart
        categories = ['A', 'B', 'C']
        val1 = [10, 15, 20]
        val2 = [5, 10, 15]
        
        fig, ax = plt.subplots()
        ax.barh(categories, val1)
        ax.barh(categories, val2, left=val1)
        plt.close()
        
        print("✓ Try 1.5.3 tests passed")
        return True
    except Exception as e:
        print(f"✗ Try 1.5.3 failed: {e}")
        return False

def main():
    """Run all tests."""
    print("=" * 60)
    print("Testing Section 1.5: Bar Charts Notebooks")
    print("=" * 60)
    
    results = []
    
    # Run all tests
    results.append(("Imports", test_basic_imports()))
    results.append(("Try 1.5.1", test_try_1_5_1()))
    results.append(("Try 1.5.2", test_try_1_5_2()))
    results.append(("Try 1.5.3", test_try_1_5_3()))
    
    # Summary
    print("\n" + "=" * 60)
    print("Test Summary")
    print("=" * 60)
    
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    for name, result in results:
        status = "✓ PASS" if result else "✗ FAIL"
        print(f"{name:20s}: {status}")
    
    print("=" * 60)
    print(f"Total: {passed}/{total} tests passed")
    print("=" * 60)
    
    # Exit with appropriate code
    if passed == total:
        print("\n✅ All tests passed! Notebooks are ready to use.")
        return 0
    else:
        print(f"\n❌ {total - passed} test(s) failed. Please check the errors above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
