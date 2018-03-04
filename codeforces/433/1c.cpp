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
typedef pair<PI, PI> PPIPI;

const int N = 1 << 18;
int bit[N];

void update(int k, int v) {
  while (k < N) {
    bit[k] += v;
    k += k & -k;
  }
}


int get(int r) {
  int tot = 0;
  while (r > 0) {
    tot += bit[r];
    r &= r - 1;
  }
  return tot;
}

int phase = 0;

int n;
VI ql, qr, qd, qu;
VI ans;
int cnt = 0;
VI p;


int get(int l, int r, int d, int u) {
  PPIPI key(PI(l, r), PI(d, u));
  if (phase == 0) {
    ql.push_back(l);
    qr.push_back(r);
    qd.push_back(d);
    qu.push_back(u);
    ans.push_back(0);
    return 0;
  }
  int idx = cnt++;
  return ans[idx];
}

void answer_to_queries(void) {
  vector<PPIPI> pool;
  REP(i, 0, ql.size()) {
    pool.push_back(PPIPI(PI(ql[i] + 1, 1), PI(i, -1)));
    pool.push_back(PPIPI(PI(qr[i] + 1, -1), PI(i, 1)));
  }
  REP(i, 0, n) {
    pool.push_back(PPIPI(PI(i + 1, 2), PI(p[i] + 1, 1)));
  }
  sort(pool.begin(), pool.end());
  REP(i, 0, pool.size()) {
    PPIPI que = pool[i];
    if (que.first.second == 2) {
      update(que.second.first, 1);
    } else {
      int fac = que.second.second;
      int idx = que.second.first;
      ans[idx] += fac * (get(qu[idx]) - get(qd[idx]));
    }
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> n >> q;
  p = VI(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  VI l(q), d(q), r(q), u(q);
  REP(i, 0, q) {
    cin >> l[i] >> d[i] >> r[i] >> u[i];
    l[i]--, d[i]--;
  }
  for(phase = 0; phase < 2; ++phase){
    cnt = 0;
    REP(i, 0, q) {
      ll tot = 0;
      ll zl0d = get(0, l[i], 0, d[i]);
      ll tmp = zl0d;
      ll lndn = get(l[i], n, d[i], n);
      tmp *= lndn;
      tot += tmp;
      ll zldu = get(0, l[i], d[i], u[i]);
      tmp = zldu;
      ll ln0n = get(l[i], n, 0, n);
      tmp *= ln0n;
      tot += tmp;
      ll ln0u = get(l[i], n, 0, u[i]);
      tmp = get(0, l[i], 0, n) - zl0d - zldu;
      tmp *= ln0u;
      tot += tmp;
      ll lr0d = get(l[i], r[i], 0, d[i]);
      tmp = lr0d;
      tmp *= lndn;
      tot += tmp;
      ll ln0d = ln0n - lndn;
      ll rn0d = ln0d - lr0d;
      ll rndu = get(r[i], n, d[i], u[i]);
      ll lnun = ln0n - ln0u;
      ll lrun = get(l[i], r[i], u[i], n);
      ll rnun = lnun - lrun;
      ll lndu = ln0u + lndn - ln0n;
      ll lrdu = lndu - rndu;
      tmp = lrdu;
      tmp *= (rn0d + rndu + rnun + lrun); // get(r[i], n, 0, n) + get(l[i], r[i], u[i], n));
      tot += tmp;
      tmp = lrun;
      tmp *= rn0d + rndu;
      tot += tmp;
      tmp = lrdu;
      tot += tmp * (tmp - 1) / 2;
      if (phase == 1) {
	cout << tot << "\n";
      }
    }
    if (phase == 0) {
      answer_to_queries();
    }
  }
}
