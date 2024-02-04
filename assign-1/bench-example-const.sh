python bench_examples.py constants > const_analysis.out
python bench_examples.py constants_analysis --json > const_analysis2.out
diff const_analysis.out const_analysis2.out
rm const_analysis.out const_analysis2.out
