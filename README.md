Channel Number | Channel Name      | Memory Address | Value 1 | Value 2 | Description
---------------|-------------------|----------------|---------|---------|-----------------------------
50000          | uln_0             | 20008830       | 17251   | 4363    | Voltage phase-to-neutral for phase 0
50002          | uln_1             | 20008834       | 17251   | 4363    | Voltage phase-to-neutral for phase 1
50004          | uln_2             | 20008838       | 17254   | 4366    | Voltage phase-to-neutral for phase 2
50006          | ull_0             | 2000883c       | 17348   | 43c4    | Voltage phase-to-phase for phase 0
50008          | ull_1             | 20008840       | 17350   | 43c6    | Voltage phase-to-phase for phase 1
50010          | ull_2             | 20008844       | 17350   | 43c6    | Voltage phase-to-phase for phase 2
50012          | iln_0             | 20008848       | 16996   | 4264    | Current for phase 0
50014          | iln_1             | 2000884c       | 16996   | 4264    | Current for phase 1
50016          | iln_2             | 20008850       | 16996   | 4264    | Current for phase 2
50018          | iln_3             | 20008854       | 0       | 0       | Current for phase 3 (if available)
50020          | pln_0             | 20008858       | 17995   | 464b    | Active power for phase 0
50022          | pln_1             | 2000885c       | 17994   | 464a    | Active power for phase 1
50024          | pln_2             | 20008860       | 17997   | 464d    | Active power for phase 2
50026          | p_sum3            | 20008864       | 18200   | 4718    | Sum of active power for all three phases
50028          | qln_0             | 20008868       | 50113   | c3c1    | Reactive power for phase 0
50030          | qln_1             | 2000886c       | 50154   | c3ea    | Reactive power for phase 1
50032          | qln_2             | 20008870       | 50146   | c3e2    | Reactive power for phase 2
50034          | q_sum3            | 20008874       | 50339   | c4a3    | Sum of reactive power for all three phases
50036          | sln_0             | 20008878       | 17995   | 464b    | Apparent power for phase 0
50038          | sln_1             | 2000887c       | 17994   | 464a    | Apparent power for phase 1
50040          | sln_2             | 20008880       | 17997   | 464d    | Apparent power for phase 2
50042          | s_sum3            | 20008884       | 18200   | 4718    | Sum of apparent power for all three phases
50044          | cosphi_0          | 20008888       | 16255   | 3f7f    | Power factor for phase 0
50046          | cosphi_1          | 2000888c       | 16255   | 3f7f    | Power factor for phase 1
50048          | cosphi_2          | 20008890       | 16255   | 3f7f    | Power factor for phase 2
50050          | cos_sum3          | 20008894       | 16255   | 3f7f    | Sum of power factors for all three phases
50052          | freq              | 20008898       | 16968   | 4248    | Frequency of the voltage
50054          | i_sum3            | 2000889c       | 16421   | 4025    | Sum of current for all three phases
50056          | pt100_actual      | 200088a0       | 18303   | 477f    | Actual temperature measured by PT100 sensor
50058          | pt100_average     | 200088a4       | 18303   | 477f    | Average temperature measured by PT100 sensor
50060          | pt100_min         | 200088a8       | 18303   | 477f    | Minimum temperature measured by PT100 sensor
50062          | pt100_max         | 200088ac       | 18303   | 477f    | Maximum temperature measured by PT100 sensor
50064          | rcm_actual        | 200088b0       | 0       | 0       | Actual residual current monitoring
50066          | rcm_limit         | 200088b4       | 0       | 0       | Residual current monitoring limit
50068          | rcm_curr_p        | 200088b8       | 0       | 0       | Residual current monitoring current peak
50070          | rcm_limit_p       | 200088bc       | 0       | 0       | Residual current monitoring limit peak
50072          | ei1_i_0           | 200088c0       | 0       | 0       | Energy input 1 for channel 0
50074          | ei1_i_1           | 200088c4       | 0       | 0       | Energy input 1 for channel 1
50076          | ei1_i_2           | 200088c8       | 0       | 0       | Energy input 1 for channel 2
50078          | ei1_i_3           | 200088cc       | 0       | 0       | Energy input 1 for channel 3
50080          | thd_uln0          | 200088d0       | 16340   | 3fd4    | Total harmonic distortion for voltage phase 0
50082          | thd_uln1          | 200088d4       | 16316   | 3fbc    | Total harmonic distortion for voltage phase 1
50084          | thd_uln2          | 200088d8       | 16322   | 3fc2    | Total harmonic distortion for voltage phase 2
50086          | thd_il0           | 200088dc       | 16428   | 402c    | Total harmonic distortion for current phase 0
50088          | thd_il1           | 200088e0       | 16428   | 402c    | Total harmonic distortion for current phase 1
50090          | thd_il2           | 200088e4       | 16432   | 4030    | Total harmonic distortion for current phase 2
50092          | g_wh0             | 200088e8       | 19375   | 4baf    | Energy consumption (wh) for phase 0
50094          | g_wh1             | 200088ec       | 19365   | 4ba5    | Energy consumption (wh) for phase 1
50096          | g_wh2             | 200088f0       | 19370   | 4baa    | Energy consumption (wh) for phase 2
50098          | g_wh_suml13       | 200088f4       | 19583   | 4c7f    | Sum of energy consumption (wh) for all three phases
50100          | g_iqh0            | 200088f8       | 17008   | 4270    | Instantaneous reactive power for phase 0
50102          | g_iqh1            | 200088fc       | 16776   | 4188    | Instantaneous reactive power for phase 1
50104          | g_iqh2            | 20008900       | 17274   | 437a    | Instantaneous reactive power for phase 2
50106          | g_iqh_suml13      | 20008904       | 17316   | 43a4    | Sum of instantaneous reactive power for all three phases
50108          | ed1_di1_c         | 20008908       | 0       | 0       | Energy data input 1 counter
50110          | ed1_di2_c         | 2000890c       | 0       | 0       | Energy data input 2 counter
50112          | ed1_di3_c         | 20008910       | 0       | 0       | Energy data input 3 counter
50114          | ed1_di4_c         | 20008914       | 0       | 0       | Energy data input 4 counter
50116          | do_out            | 20008918       | 0       | 0       | Digital output
50117          | di_in             | 2000891a       | 0       | 0       | Digital input
50118          | module_state      | 2000891c       | 4       | 4       | Module state
50119          | date_ym           | 2000891e       | 6151    | 1807    | Date in year-month format
50120          | date_dh           | 20008920       | 528     | 210     | Date in day-hour format
50121          | date_ms           | 20008922       | 13869   | 362d    | Date in minutes-seconds format
50122          | basis_ver         | 20008924       | 8218    | 201a    | Basic version of the firmware/software
29996          | i4ct1             | 20008928       | 5       | 5       | Current transformer 1 input 4
29997          | i4ct2             | 2000892a       | 5       | 5       | Current transformer 2 input 4
29998          | i5ct1             | 2000892c       | 700     | 2bc     | Current transformer 1 input 5
29999          | i5ct2             | 2000892e       | 1       | 1       | Current transformer 2 input 5
30000          | basis_addr        | 20008930       | 1       | 1       | Basic address
30001          | basis_baud        | 20008932       | 4       | 4       | Basic baud rate
30002          | basis_format      | 20008934       | 0       | 0       | Basic format
30003          | reserv0           | 20008936       | 0       | 0       | Reserved
30004          | wiring            | 20008938       | 0       | 0       | Wiring configuration
30005          | grid_freq         | 2000893a       | 0       | 0       | Grid frequency
30006          | reserv1           | 2000893c       | 0       | 0       | Reserved
30007          | reserv2           | 2000893e       | 0       | 0       | Reserved
30008          | pt1               | 20008940       | 0       | 0       | Potential transformer 1
30010          | ct1               | 20008944       | 0       | 0       | Current transformer 1
30012          | pt2               | 20008948       | 230     | e6      | Potential transformer 2
30013          | ct2               | 2000894a       | 5       | 5       | Current transformer 2
30014          | rcm_limit         | 2000894c       | 2       | 2       | Residual current monitoring limit
30015          | temp_comp         | 2000894e       | 0       | 0       | Temperature compensation

