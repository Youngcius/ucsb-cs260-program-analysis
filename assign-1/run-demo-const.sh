# constants-noptr-call.lir    constants-ptr-nocall.lir       test2.lir  test5.lir
# constants-noptr-nocall.lir  test3.lir  test6.lir
# constants-ptr-call.lir       test1.lir                 test4.lir

./constants ./demos/lir/test1.lir test > ./demos/std_const_out/test1.const.out
./constants ./demos/lir/test2.lir test > ./demos/std_const_out/test2.const.out
./constants ./demos/lir/test3.lir test > ./demos/std_const_out/test3.const.out
./constants ./demos/lir/test6.lir test > ./demos/std_const_out/test6.const.out
./constants ./demos/lir/constants-noptr-call.lir test > ./demos/std_const_out/constants-noptr-call.const.out
./constants ./demos/lir/constants-noptr-nocall.lir test > ./demos/std_const_out/constants-noptr-nocall.const.out
./constants ./demos/lir/constants-ptr-call.lir test > ./demos/std_const_out/constants-ptr-call.const.out
./constants ./demos/lir/constants-ptr-nocall.lir test > ./demos/std_const_out/constants-ptr-nocall.const.out




./constants_analysis ./demos/json/test1.json test > ./demos/const_out/test1.const.out
./constants_analysis ./demos/json/test2.json test > ./demos/const_out/test2.const.out
./constants_analysis ./demos/json/test3.json test > ./demos/const_out/test3.const.out
./constants_analysis ./demos/json/test6.json test > ./demos/const_out/test6.const.out
./constants_analysis ./demos/json/constants-noptr-call.json test > ./demos/const_out/constants-noptr-call.const.out
./constants_analysis ./demos/json/constants-noptr-nocall.json test > ./demos/const_out/constants-noptr-nocall.const.out
./constants_analysis ./demos/json/constants-ptr-call.json test > ./demos/const_out/constants-ptr-call.const.out
./constants_analysis ./demos/json/constants-ptr-nocall.json test > ./demos/const_out/constants-ptr-nocall.const.out
