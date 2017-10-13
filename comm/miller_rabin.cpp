// 0 <= x < mod, 0 <= y, mod < 2^62
long long mulmod(long long x, long long y, long long mod) {
  long long sum = 0;
  long long cur = x;
  while (y > 0) {
    if (y % 2 == 1) {
      sum += cur;
      if (sum >= mod) {
	sum -= mod;
      }
    }
    cur *= 2;
    if (cur >= mod) {
      cur -= mod;
    }
    y /= 2;
  }
  return sum;
  
}

// 1 <= a < mod, 0 <= e, mod < 2^62
long long powmod(long long a, long long e, long long mod) {
  long long sum = 1;
  long long cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = mulmod(sum, cur, mod);
    }
    cur = mulmod(cur, cur, mod);
    e /= 2;
  }
  return sum;
}

// x < 2^62
bool is_prime(long long x) {
  if (x <= 1) { return false; }
  if (x <= 3) { return true; }
  if (x % 2 == 0) {
    return false;
  }
  // From https://milong longer-rabin.appspot.com/
  long long bases[7] = {2, 325, 9375, 28178, 450775, 9780504, 1795265022};
  int e = 0;
  long long d = x - 1;
  while (d % 2 == 0) {
    e += 1;
    d /= 2;
  }
  for (int i = 0; i < 7; ++i) {
    long long r = bases[i] % x;
    if (r == 0) {
      continue;
    }
    long long t = powmod(r, d, x);
    bool ok = true;
    for (int j = 0; j < e; ++j) {
      if (t == 1) { break; }
      long long u = mulmod(t, t, x);
      if (u == 1 && t != x - 1) {
	ok = false;
	break;
      }
      t = u;
    }
    ok &= t == 1;
    if (not ok) {
      return false;
    }
  }
  return true;
}
