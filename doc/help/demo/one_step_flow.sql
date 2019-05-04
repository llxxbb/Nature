INSERT INTO one_step_flow (from_thing,from_version,to_thing,to_version,settings) VALUES (
'/B/multi_downstream/from',1,'/B/multi_downstream/toA',1,'{"executor":[{"protocol":"LocalRust","url":"local://multi_downstream","proportion":1}]}');
INSERT INTO one_step_flow (from_thing,from_version,to_thing,to_version,settings) VALUES (
'/B/multi_downstream/from',1,'/B/multi_downstream/toB',1,'{"executor":[{"protocol":"LocalRust","url":"local://multi_downstream","proportion":1}]}');
INSERT INTO one_step_flow (from_thing,from_version,to_thing,to_version,settings) VALUES (
'/B/local_converter/from',1,'/B/local_converter/to',1,'{"executor":[{"protocol":"LocalRust","url":"nature_integrate_test_converter.dll:rtn_one","proportion":1}]}');
