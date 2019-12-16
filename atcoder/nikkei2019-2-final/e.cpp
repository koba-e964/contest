#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, PI> PIPI;
typedef pair<PI, PI> PPIPI;

#define TESTING 0

const int DEBUG = 0;

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */
template<class I, class BiOp>
class SegTree {
  int n;
  std::vector<I> dat;
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
    }
  }
  /* http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
  I querySub(int a, int b) const {
    I left = e;
    I right = e;
    a += n - 1;
    b += n - 1;
    while (a < b) {
      if ((a & 1) == 0) {
	left = op(left, dat[a]);
      }
      if ((b & 1) == 0) {
	right = op(dat[b - 1], right);
      }
      a = a / 2;
      b = (b - 1) / 2;
    }
    return op(left, right);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1);
  }
};

const int N = 200000;

set<PI> ranges;
vector<PI> lr;

//[l, r]がencloseする区間の個数を知りたい。
//l,r, どこにaffectするか,係数
vector<PPIPI> reqs;
VI ans;

VI terms[N];
VI asks[N];


// add 0
void add(int v, int idx) {
  int l = v;
  int r = v;
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

void dump(void) {
  if (DEBUG) {
    for (auto r: ranges) {
      cerr <<" " << r.first << "," << r.second;
    }
    cerr << endl;
  }
}

void solve(void) {
  int r = reqs.size();
  REP(i, 0, r) {
    int l = reqs[i].first.first;
    asks[l].push_back(i); // query appears
  }
  int m = lr.size();
  SegTree<int, plus<int> > st(N + 1, plus<int>(), 0);
  for (int l = N - 1; l >= 0; l--) {
    for (int r: terms[l]) {
      if (DEBUG) {
        cerr << l << "," << r << " appears" << endl;
      }
      st.update(r, st.query(r, r) + 1);
    }
    for (int idx: asks[l]) {
      int r = reqs[idx].first.second;
      PI info = reqs[idx].second;
      int res = st.query(l, r + 1);
      if (DEBUG) {
        cerr << "query " << info.first << " " << info.second << " " << res;
        cerr << endl;
      }
      ans[info.first] -= info.second * res;
    }
  }
}

// The author read the editorial before implementing it.
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, q;
  cin >> n >> m >> q;
  string s;
#if TESTING
  s = string(n, '0');
#else
  cin >> s;
#endif

  VI acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + (s[i] == '1');
  int cur = -1;
  REP(i, 0, n + 1) {
    if (i < n && s[i] == '0') {
      if (cur == -1) cur = i;
    } else {
      if (cur >= 0)
        ranges.insert(PI(cur, i - 1));
      cur = -1;
    }
  }

  int init = 0;
  REP(i, 0, m) {
    int l, r;
#if TESTING
    l = 1, r = n;
#else
    cin >> l >> r;
#endif
    l--;
    lr.push_back(PI(l, r));
    init += acc[r] > acc[l];
    terms[l].push_back(r);
  }
  dump();
  VI x(q);
  REP(i, 0, q) {
#if TESTING
    x[i] = 1;
#else
    cin >> x[i];
#endif
    x[i]--;
    if (has(x[i])) {
      del(x[i], i);
    } else {
      add(x[i], i);
    }
    if (DEBUG) {
      cerr << i << endl;
      dump();
    }
  }
  if (DEBUG) {
    REP(i, 0, reqs.size()) {
      cerr << i << ":" << reqs[i].first.first << " " << reqs[i].first.second
           << " " << reqs[i].second.first << " " << reqs[i].second.second
           << endl;
    }
  }
  ans = VI(q, 0);
  solve();
  REP(i, 0, q) {
    ans[i] += i == 0 ? init : ans[i - 1];
    cout << ans[i] << "\n";
  }
}
