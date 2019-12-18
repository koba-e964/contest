/*
 * Manages intervals with no intersections.
 * Header dependencies: set, vector
 * Verified by: https://atcoder.jp/contests/nikkei2019-2-final/submissions/8999576
 */

set<PI> ranges;

//[l, r]がencloseする区間の個数を知りたい。
//l,r, どこにaffectするか,係数
vector<PPIPI> reqs;

// 区間 [l, r] を加える。元の区間と重複しないことが必要。
void add(int l, int r, int idx) {
  auto it = ranges.lower_bound(PI(l, 1e8));
  if (it != ranges.begin()) {
    it--;
    PI t = *it;
    assert (t.second < l);
    if (t.second == l - 1) {
      ranges.erase(it);
      reqs.push_back(PPIPI(t, PI(idx, -1)));
      l = t.first;
    }
  }
  it = ranges.lower_bound(PI(r, 1e8));
  if (it != ranges.end()) {
    PI t = *it;
    assert (t.first > r);
    if (t.first == r + 1) {
      ranges.erase(it);
      reqs.push_back(PPIPI(t, PI(idx, -1)));
      r = t.second;
    }
  }
  ranges.insert(PI(l, r));
  reqs.push_back(PPIPI(PI(l, r), PI(idx, 1)));
}

// {v} を取り除く。元の区間に含まれることが必要。
void del(int v, int idx) {
  auto it = ranges.lower_bound(PI(v, 1e9));
  assert (it != ranges.begin());
  it--;
  PI t = *it;
  ranges.erase(it);
  reqs.push_back(PPIPI(t, PI(idx, -1)));
  if (t.first < v) {
    ranges.insert(PI(t.first, v - 1));
    reqs.push_back(PPIPI(PI(t.first, v - 1), PI(idx, 1)));
  }
  if (t.second > v) {
    ranges.insert(PI(v + 1, t.second));
    reqs.push_back(PPIPI(PI(v + 1, t.second), PI(idx, 1)));
  }
}

bool has(int v) {
  auto it = ranges.lower_bound(PI(v, 1e9));
  if (it == ranges.begin()) return false;
  it--;
  PI t = *it;
  return t.first <= v && v <= t.second;
}
