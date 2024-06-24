jchess_out = """g4g3: 4781
g4a4: 4640
g4b4: 4643
g4c4: 5643
g4d4: 5262
g4e4: 4379
g4f4: 1520
g4h4: 3742
g4g5: 1297
g4g6: 3639
g4g7: 4597
g4g8: 5284
a5a4: 4238
a5b4: 4322
a5a6: 4414
e2e3: 3942
g2g3: 3696
b5b6: 4225
e2e4: 686"""

fish_out = """e2e3: 3942
g2g3: 3695
b5b6: 4224
e2e4: 686
g4g3: 4780
g4a4: 4639
g4b4: 4642
g4c4: 5642
g4d4: 5261
g4e4: 4379
g4f4: 1520
g4h4: 3741
g4g5: 1297
g4g6: 3638
g4g7: 4596
g4g8: 5283
a5a4: 4237
a5b4: 4321
a5a6: 4413"""

for line in jchess_out.splitlines():
    if line not in fish_out.splitlines():
        print(line)

