# constants-noptr-call.lir    constants-ptr-nocall.lir       test2.lir  test5.lir
# constants-noptr-nocall.lir  test3.lir  test6.lir
# constants-ptr-call.lir       test1.lir                 test4.lir

./intervals ./demos/lir/test1.lir test > ./demos/std_inter_out/test1.inter.out
./intervals ./demos/lir/test2.lir test > ./demos/std_inter_out/test2.inter.out
./intervals ./demos/lir/test3.lir test > ./demos/std_inter_out/test3.inter.out
./intervals ./demos/lir/test6.lir test > ./demos/std_inter_out/test6.inter.out
./intervals ./demos/lir/intervals-noptr-call.lir test > ./demos/std_inter_out/intervals-noptr-call.inter.out
./intervals ./demos/lir/intervals-noptr-nocall.lir test > ./demos/std_inter_out/intervals-noptr-nocall.inter.out
./intervals ./demos/lir/intervals-ptr-call.lir test > ./demos/std_inter_out/intervals-ptr-call.inter.out
./intervals ./demos/lir/intervals-ptr-nocall.lir test > ./demos/std_inter_out/intervals-ptr-nocall.inter.out


./intervals_analysis ./demos/json/test1.json test > ./demos/inter_out/test1.inter.out
./intervals_analysis ./demos/json/test2.json test > ./demos/inter_out/test2.inter.out
./intervals_analysis ./demos/json/test3.json test > ./demos/inter_out/test3.inter.out
./intervals_analysis ./demos/json/test6.json test > ./demos/inter_out/test6.inter.out
./intervals_analysis ./demos/json/intervals-noptr-call.json test > ./demos/inter_out/intervals-noptr-call.inter.out
./intervals_analysis ./demos/json/intervals-noptr-nocall.json test > ./demos/inter_out/intervals-noptr-nocall.inter.out
./intervals_analysis ./demos/json/intervals-ptr-call.json test > ./demos/inter_out/intervals-ptr-call.inter.out
./intervals_analysis ./demos/json/intervals-ptr-nocall.json test > ./demos/inter_out/intervals-ptr-nocall.inter.out
