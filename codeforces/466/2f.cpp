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
#include <unordered_map>
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

const int BB = 2000;

bool cmp(PPIPI x, PPIPI y) {
#define FST(x) ((x).second.first)
#define SND(x) ((x).second.second)
#define THR(x) ((x).first.first)
  if (FST(x) / BB == FST(y) / BB) {
    if (SND(x) / BB == SND(y) / BB) {
      int odd = (SND(y) / BB + FST(y) / BB) % 2;
      if (odd) {
	return THR(x) > THR(y);
      } else {
	return THR(x) < THR(y);
      }
    }
    int odd = (FST(x) / BB) % 2;
    if (odd) {
      return SND(x) > SND(y);
    } else {
      return SND(x) < SND(y);
    }
  }
  return FST(x) < FST(y);
}

// Global states
const int W = 110000;
int cutime;
int culeft = 0;
int curight = 0;

int freq[2 * W];
int freqfreq[W];
int a[W];

void disap(int idx) {
  int val = a[idx];
  freq[val] -= 1;
  if (freq[val] >= 0) {
    freqfreq[freq[val]] += 1;
  }
  if (freq[val] >= -1) {
    freqfreq[freq[val] + 1] -= 1;
  }
}
void ap(int idx) {
  int val = a[idx];
  freq[val] += 1;
  if (freq[val] >= 0) {
    freqfreq[freq[val]] += 1;
  }
  if (freq[val] >= 1) {
    freqfreq[freq[val] - 1] -= 1;
  }
}

void disap_hedge(int idx) {
  if (idx < culeft || idx >= curight) return;
  disap(idx);
}
void ap_hedge(int idx) {
  if (idx < culeft || idx >= curight) return;
  ap(idx);
}

void debug(void) {
  cerr << "in pool:";
  cerr << "freqfreq:";
  REP(i, 0, 10) {
    cerr << " (" << i << "=>" << freqfreq[i] << ")";
  }
  cerr << endl;
}

// This solution was written after the author read a spoiler.
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  REP(i, 0, n) cin >> a[i];
  vector<PPIPI> que;
  vector<PIPI> upd;
  VI app;
  REP(i, 0, n) app.push_back(a[i]);
  REP(i, 0, q) {
    int t;
    cin >> t;
    if (t == 1) {
      int l, r;
      cin >> l >> r;
      l--;
      int time = upd.size();
      int idx = que.size();
      que.push_back(PPIPI(PI(time, idx), PI(l, r)));
    } else {
      int p, x;
      cin >> p >> x;
      p--;
      int old = a[p];
      a[p] = x;
      upd.push_back(PIPI(p, PI(old, x)));
      app.push_back(x);
    }
  }
  // coord_comp
  {
    sort(app.begin(), app.end());
    app.erase(unique(app.begin(), app.end()), app.end());
    assert ((int) app.size() <= 2 * W);
    unordered_map<int, int> tbl;
    REP(i, 0, app.size()) tbl[app[i]] = i;
    REP(i, 0, n) a[i] = tbl[a[i]];
    REP(i, 0, upd.size()) {
      upd[i].second.first = tbl[upd[i].second.first];
      upd[i].second.second = tbl[upd[i].second.second];
    }
  }
  sort(que.begin(), que.end(), cmp);
  cutime = upd.size();
  VI ans(que.size());
  REP(_, 0, que.size()) {
    int chro = que[_].first.first;
    int idx = que[_].first.second;
    int l = que[_].second.first;
    int r = que[_].second.second;
    while (cutime < chro) {
      PIPI cur = upd[cutime];
      disap_hedge(cur.first);
      a[cur.first] = cur.second.second;
      ap_hedge(cur.first);
      cutime++;
    }
    while (cutime > chro) {
      cutime--;
      PIPI cur = upd[cutime];
      disap_hedge(cur.first);
      a[cur.first] = cur.second.first;
      ap_hedge(cur.first);
    }
    while (culeft < l) {
      disap(culeft);
      culeft++;
    }
    while (culeft > l) {
      culeft--;
      ap(culeft);
    }
    while (curight < r) {
      ap(curight);
      curight++;
    }
    while (curight > r) {
      curight--;
      disap(curight);
    }
    if (0) {
      DEBUGP(chro);
      DEBUGP(idx);
      DEBUGP(l);
      DEBUGP(r);
      debug();
      cerr<<endl;
    }
    int mex = 1;
    while (freqfreq[mex] > 0) {
      mex++;
    }
    ans[idx] = mex;
  }
  REP(i, 0, ans.size()) {
    cout << ans[i] << "\n";
  }
}
