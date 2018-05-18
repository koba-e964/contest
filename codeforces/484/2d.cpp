#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <cassert>
#include <vector>
#include <map>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

map<int, int> rgt, lft;
map<int, int> freq;
int pop = 0;
int sumpop = 0;

void rm(int x, int y) {
  rgt.erase(x);
  lft.erase(y);
  freq[y - x + 1]--;
  if (freq[y - x + 1] == 0) pop--;
  sumpop--;
}

void add(int x, int y) {
  rgt[x] = y;
  lft[y] = x;
  if (freq[y - x + 1] == 0) pop++;
  freq[y - x + 1]++;
  sumpop++;
}

void assert_integrity(void) {
  vector<PI> l, r;
  for (auto e: lft) {
    l.push_back(PI(e.second, e.first));
  }
  for (auto e: rgt) {
    r.push_back(e);
  }
  assert (l == r);
  map<int, int> realfreq;
  REP(i, 0, l.size()) {
    realfreq[l[i].second - l[i].first + 1]++;
  }
  int realpop = 0;
  int realsumpop = 0;
  for (auto e: realfreq) {
    assert (e.second == freq[e.first]);
    realpop++;
    realsumpop += e.second;
  }
  assert (pop == realpop);
  assert (sumpop == realsumpop);
}

int main(void) {
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  vector<PI> pool;
  REP(i, 0, n) {
    pool.push_back(PI(a[i], i));
  }
  sort(pool.begin(), pool.end());
  PI ma(-1, -1);
  REP(i, 0, n) {
    int idx = pool[i].second;
    int x = idx, y = idx;
    if (lft.count(idx - 1)) {
      // merge
      int targ = lft[idx - 1];
      x = targ;
      rm(targ, idx - 1);
    }
    if (rgt.count(idx + 1)) {
      int targ = rgt[idx + 1];
      y = targ;
      rm(idx + 1, targ);
    }
    add(x, y);
    if (pop == 1) {
      ma = max(ma, PI(sumpop, -pool[i].first));
    }
    // assert_integrity();
  }
  cout << -ma.second + 1 << endl;
}
