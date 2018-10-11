#include <algorithm>
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

const int N = 1 << 17;
int bit[N];
void bit_init(void) {
  REP(i, 0, N) bit[i] = 0;
}
void add(int i, int x) {
  while (i < N) {
    bit[i] += x;
    i += i & -i;
  }
}

int accum(int i) {
  int sum = 0;
  while (i > 0) {
    sum += bit[i];
    i &= i - 1;
  }
  return sum;
}

// returns argmax_i accum(i) <= v, or 0 if v <= 0
int bisect(int v) {
  if (v <= 0) return 0;
  int p = 0;
  int acc = 0;
  for (int i = 16; i >= 0; --i) {
    if (acc + bit[p + (1 << i)] <= v) {
      p += 1 << i;
      acc += bit[p];
    }
  }
  return p;
}


vector<PI> qs[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  VL a(n); REP(i, 0, n) cin >> a[i];
  VL s(k); REP(i, 0, k) cin >> s[i];
  sort(s.begin(), s.end());
  VI l(m), r(m), x(m);
  REP(i, 0, m) {
    cin >> l[i] >> r[i] >> x[i];
    l[i]--;
    qs[l[i]].push_back(PI(i + 1, x[i]));
    qs[r[i]].push_back(PI(i + 1, -x[i]));
  }
  VI stat(n, -1);
  vector<VI> res(m + 1);
  VL ans(m + 1, 0);
  REP(i, 0, n) {
    for (PI e: qs[i]) {
      int qidx = e.first;
      int val = e.second;
      add(qidx, val);
    }
    int targ = -1;
    REP(j, 0, k) {
      int dif = s[j] - a[i];
      int idx = bisect(dif);
      /*
      cerr << "(i,j)=" << i << " " << j << endl;
      DEBUGP(dif);
      DEBUGP(idx);
      DEBUGP(accum(idx));
      */
      if (accum(idx) == dif) {
        targ = idx;
        break;
      }
    }
    stat[i] = targ == -1 ? m + 1 : targ + 1;
    //DEBUGP(targ);
    if (stat[i] <= m) {
      res[stat[i]].push_back(i);
    }
  }
  bit_init();
  REP(i, 0, n) add(i + 1, 1);
  ll sum = 0;
  REP(i, 0, n) sum += a[i];
  REP(i, 0, m) {
    for (int e: res[i + 1]) {
      add(e + 1, -1);
      assert (accum(e + 1) == accum(e));
    }
    int val = x[i] * (accum(r[i]) - accum(l[i]));
    sum += val;
    ans[i] = sum;
    //DEBUGP(val);
    //DEBUGP(sum);
  }
  REP(i, 0, m) cout << ans[i] << endl;
}
