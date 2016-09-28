std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}
