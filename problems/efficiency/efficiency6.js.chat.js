// Prime checking function
function isPrime(n) {
  function isDivisibleByAny(current) {
    if (current === n) {
      return true;
    } else {
      if (n % current === 0) {
        return false;
      } else {
        return isDivisibleByAny(current + 1);
      }
    }
  }
  return isDivisibleByAny(2);
}

// Fibonacci number generator
function fibonacci(n) {
  function fibRecursive(index) {
    if (index < 2) {
      return 1;
    } else {
      return fibRecursive(index - 1) + fibRecursive(index - 2);
    }
  }
  return fibRecursive(n);
}

// Main function
function computeResult() {
  // Start with an initial value for computations
  const initialValue = 2;

  // Function composition and application
  function applyTwice(func, arg) {
    return func(func(arg));
  }

  // Search for a number greater than 30 satisfying conditions
  function findNumberGreaterThan30(checkFunc) {
    // checkFunc = fibonacci
    function checkCondition(current) {
      if (current > 30 && isPrime(applyTwice(checkFunc, current))) {
        return current;
      } else {
        return checkCondition(current + 1);
      }
    }
    return checkCondition(initialValue);
  }

  // Use Fibonacci function to compute the final result
  return applyTwice(findNumberGreaterThan30, fibonacci);
}

// Example usage:
const result = computeResult();
console.log("Result:", result);
