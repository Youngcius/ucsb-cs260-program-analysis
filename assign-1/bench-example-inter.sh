python bench_examples.py intervals > inter_analysis.out
python bench_examples.py intervals_analysis --json > inter_analysis2.out
diff inter_analysis.out inter_analysis2.out
# rm inter_analysis.out inter_analysis2.out
