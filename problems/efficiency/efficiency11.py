def check_conditions(var_1):
    var_12 = 1 + (var_1 // 6561 % 9)
    var_13 = 1 + (var_1 // 59049 % 9)
    var_17 = 1 + (var_1 // 205891132094649 % 9)
    var_25 = 1 + (var_1 // 7625597484987 % 9)
    var_28 = 1 + (var_1 // 2541865828329 % 9)
    var_29 = 1 + (var_1 // 22876792454961 % 9)
    var_33 = 1 + (var_1 // 31381059609 % 9)
    var_41 = 1 + (var_1 // 387420489 % 9)
    var_42 = 1 + (var_1 // 4782969 % 9)
    var_45 = 1 + (var_1 // 205891132094649 % 9)
    var_54 = 1 + (var_1 // 387420489 % 9)
    var_57 = 1 + (var_1 // 2541865828329 % 9)
    var_59 = 1 + (var_1 // 7625597484987 % 9)
    var_71 = 1 + (var_1 // 31381059609 % 9)
    var_81 = 1 + (var_1 // 205891132094649 % 9)
    var_88 = 1 + (var_1 // 6561 % 9)
    var_94 = 1 + (var_1 // 59049 % 9)
    
    return (
        var_12 == 6 and var_13 == 4 and var_17 == 7 and var_25 == 2 and
        var_28 == 3 and var_29 == 6 and var_33 == 1 and var_41 == 2 and
        var_42 == 3 and var_45 == 8 and var_54 == 7 and var_57 == 1 and
        var_59 == 4 and var_71 == 9 and var_81 == 8 and var_88 == 2 and
        var_94 == 4
    )

var_1 = 0
while not check_conditions(var_1):
    var_1 += 1

print(var_1)
