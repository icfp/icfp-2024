def check_conditions(var_1):
    conditions = {
        12: 6,
        13: 4,
        17: 7,
        25: 2,
        28: 3,
        29: 6,
        33: 1,
        41: 2,
        42: 3,
        45: 8,
        54: 7,
        57: 1,
        59: 4,
        71: 9,
        81: 8,
        88: 2,
        94: 4
    }
    
    for key, value in conditions.items():
        if (1 + ((var_1 // 9**(key-12)) % 9)) != value:
            return False
    return True

var_1 = 0
while not check_conditions(var_1):
    var_1 += 1

print(var_1)
