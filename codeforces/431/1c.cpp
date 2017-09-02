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
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<PI, PI> PIPI;

struct mo_cmp {
  bool operator()(PIPI a, PIPI b) const {
    PI aidx(a.first.first / 2000, a.second.second / 2000);
    PI bidx(b.first.first / 2000, b.second.second / 2000);
    if (aidx != bidx) {
      return aidx < bidx;
    }
    int turner = aidx.second % 2;
    return (turner ? -1 : 1) * (a.second.first - b.second.first) < 0;
  }
};

const int DEBUG = 0;

const int N = 123456;

int mi[N], ma[N];
int prevv[N], nextt[N];

ll tot = 0;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI a(n);
  vector<set<int> > occur(n + 1);
  REP(i, 0, n) {
    cin >> a[i];
    prevv[i] = nextt[i] = -1;
    int ma = -1;
    if (occur[a[i]].size() > 0) {
      ma = *occur[a[i]].rbegin();
    }
    occur[a[i]].insert(i);
    if (ma != -1) {
      nextt[ma] = i;
      prevv[i] = ma;
    }
  }
  vector<PI> tempo;
  vector<vector<PI> > tempo_aux;
  vector<vector<PI> > tempo_rev_aux;
  vector<PIPI> que;
  REP(i, 0, m) {
    int kind, p, x;
    cin >> kind >> p >> x;
    if (kind == 1) {
      p--;
      tempo.push_back(PI(p, x));
    } else {
      int time = tempo.size();
      p--;
      que.push_back(PIPI(PI(time, que.size()), PI(p, x)));
    }
  }
  vector<PI> tempo_rev(tempo.size());
  REP(i, 0, tempo.size()) {
    PI q = tempo[i];
    int p = q.first;
    int x = q.second;
    int oldv = a[p];
    tempo_rev[i] = PI(p, oldv);
    a[p] = x;
    vector<PI> opseq;
    vector<PI> invopseq;
#define MODIFY_NEXT(idx, val) do {\
	invopseq.push_back(PI((idx) + n, nextt[idx]));\
	nextt[idx] = val;			      \
	opseq.push_back(PI((idx) + n, val));	      \
      } while (0)
#define MODIFY_PREV(idx, val) do {		      \
	invopseq.push_back(PI((idx), prevv[idx]));    \
        prevv[idx] = val;			      \
	opseq.push_back(PI((idx), val));	      \
      } while (0)
    if (oldv != x) {
      if (nextt[p] >= 0) {
	MODIFY_PREV(nextt[p], prevv[p]);
      }
      if (prevv[p] >= 0) {
	// If modifying nextt, index has a bias of n.
	MODIFY_NEXT(prevv[p], nextt[p]);
      }
      // modification to x
      occur[oldv].erase(p);
      occur[x].insert(p);
      set<int>::iterator it = occur[x].find(p);
      int pr = -1, nx = -1;
      if (it != occur[x].begin()) {
	it--;
	pr = *it;
	it++;
      }
      it++;
      if (it != occur[x].end()) {
	nx = *it;
      }
      MODIFY_PREV(p, pr);
      if (pr != -1) {
	MODIFY_NEXT(pr, p);
      }
      MODIFY_NEXT(p, nx);
      if (nx != -1) {
	MODIFY_PREV(nx, p);
      }
    }
    tempo_aux.push_back(opseq);
    tempo_rev_aux.push_back(invopseq);
  }
  sort(que.begin(), que.end(), mo_cmp());
  int cur = tempo.size();
  int cl = 0;
  int cr = 0;
  // states
  if (DEBUG) {
    cerr << "Mo done" << endl;
  }
  VL ans(que.size());
  REP(i, 0, que.size()) {
    if (DEBUG) {
      cerr << endl;
      cerr << "round " << i << ":" << endl;
      cerr << "cur = " << cur << " cl = " << cl << " cr = " << cr << endl;
      cerr << "tot = " << tot << endl;
      cerr << "a:";
      REP(i, 0, n) {
	cerr << " " << a[i];
      }
      cerr << endl;
      REP(i, 0, n) {
	cerr << "prevnext[" << i << "]: " << prevv[i] << " " << nextt[i] << endl;
      }
    }
    PIPI q = que[i];
    int time = q.first.first;
    int query_idx = q.first.second;
    int l = q.second.first;
    int r = q.second.second; // [l, r)
    while (cur != time) {
      PI delta = cur < time ? tempo[cur] : tempo_rev[cur - 1];
      vector<PI> pnops = cur < time ? tempo_aux[cur] : tempo_rev_aux[cur - 1];
      int p = delta.first;
      int x = delta.second;
      a[p] = x;
      ll double_tot = 0;
      for (PI op: pnops) {
	int idx = op.first;
	int val = op.second;
	if (DEBUG) {
	  cerr << "pnop " << idx << " " << val << endl;
	  cerr << "tot: " << double_tot << " -> ";
	}
	if (idx >= n) {
	  idx -= n;
	  int old = nextt[idx];
	  if (idx >= cl && old != -1 && old < cr) {
	    double_tot -= old - idx;
	  }
	  nextt[idx] = val;
	  if (idx >= cl && val != -1 && val < cr) {
	    double_tot += val - idx;
	  }
	} else {
	  int old = prevv[idx];
	  if (old != -1 && old >= cl && idx < cr) {
	    double_tot -= idx - old;
	  }
	  prevv[idx] = val;
	  if (val != -1 && val >= cl && idx < cr) {
	    double_tot += idx - val;
	  }
	}
	if (DEBUG) {
	  cerr << double_tot << endl;
	}
      }
      assert (double_tot % 2 == 0);
      tot += double_tot / 2;
      cur += cur < time ? 1 : -1;
    }
    while (cr != r) {
      bool lt = cr < r;
      if (not lt) {
	cr -= 1;
      }
      if (prevv[cr] != -1 && prevv[cr] >= cl) {
	tot += (lt ? 1 : -1) * (cr - prevv[cr]);
      }
      if (lt) {
	cr += 1;
      }
    }
    while (cl != l) {
      bool lt = cl < l;
      if (not lt) {
	cl -= 1;
      }
      if (nextt[cl] != -1 && nextt[cl] < cr) {
	tot += (lt ? -1 : 1) * (nextt[cl] - cl);
      }
      if (lt) {
	cl += 1;
      }
    }
    ans[query_idx] = tot;
  }
  REP(i, 0, que.size()) {
    cout << ans[i] << endl;
  }
}
