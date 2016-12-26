void factorize(long long v, std::map<long long, int> &result) {
  long long p = 2;
  while (v > 1 || p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      if (result.count(p) == 0) {
	result[p] = 0;
      }
      result[p] += cnt;
    }
    p += p == 2 ? 1 : 2;
  }
}

std::pair<std::vector<int>, long long>
factor_base(long long v, const std::vector<long long> &base) {
  int n = base.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    long long p = base[i];
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    ret[i] = cnt;
  }
  return std::pair<std::vector<int>, long long>(ret, v);
}
