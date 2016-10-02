std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}

// q after p
std::vector<int> perm_comp(const std::vector<int> &q, const std::vector<int> &p) {
  int n = p.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    ret[i] = q[p[i]];
  }
  return ret;
}
