(function lam_0(isPrime) {
  return (function lam_1(var_7) {
    return (function lam_2(f) {
      return (function lam_3(self) {
        return f(self(self))
      })((function lam_4(self) {
        return f(self(self))
      }))
    })(function lam_5(recurse) {
      return (function lam_6(n) {
        if (((n > 30) && isPrime(var_7(n)))) {
          return n
        } else {
          return recurse((n + 1))
        }
      })
    })(2)
  })((function yComb_1(var_1) {
    return (function lam_8(var_2) {
      return var_1(var_2(var_2))
    })((function lam_9(var_2) {
      return var_1(var_2(var_2))
    }))
  })(function fibRecurse(recurse) {
    return (function fib(n) {
      if ((n < 2)) {
        return 1
      } else {
        return (recurse((n - 1)) + recurse((n - 2)))
      }
    })
  }))
})(function isPrime(candidate) {
  return (function lam_13(f) {
    return (function yComb(self) {
      return f(self(self))
    })((function lam_15(var_2) {
      return f(var_2(var_2))
    }))
  })((function isPrimeRecurse(recurse) {
    return (function isPrime(n) {
      if ((n == candidate)) {
        return true
      } else {
        if (((candidate % n) == 0)) {
          return false
        } else {
          return recurse((n + 1))
        }
      }
    })
  }))(2)
})
