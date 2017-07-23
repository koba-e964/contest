#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, int> PLI;
typedef pair<ll, ll> PL;

const ll inf = 5e15;

int smain(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int x, y, z;
  cin >> x >> y >> z;
  vector<VL> a(x + y + z, VL(3));
  ll tot = 0;
  vector<PL> pool;
  REP(i, 0, x + y + z) {
    REP(j, 0, 3) {
      cin >> a[i][j];
    }
    tot += a[i][0];
    pool.push_back(PLI(a[i][0] - a[i][1], a[i][0] - a[i][2]));
  }
  // min cost to pay
  sort(pool.begin(), pool.end());
  // Try picking first (y + z) elems
  REP(i, 0, y + z) {
    tot -= pool[i].first;
  }
  cerr << "tot = " << tot << endl;
  // improve
  VI status(x + y + z, 0);
  priority_queue<pair<ll, PI>, vector<pair<ll, PI> >, greater<pair<ll, PI> > > que;
  priority_queue<ll, VL, greater<ll> > zq;
  REP(i, y + z, x + y + z) {
    zq.push(pool[i].second);
  }
  vector<PLI> ops;
  REP(i, 0, y + z) {
    ops.push_back(PLI(pool[i].second - pool[i].first, i));
  }
  sort(ops.begin(), ops.end());
  int pos = 0;
  int ypos = y + z - 1;
  int rem = z;
  while (rem > 0 && (pos < (int) ops.size() || (not zq.empty() && ypos >= 0))) {
    ll qdif = inf;
    if (not zq.empty()) {
      while (ypos >= 0) {
	if (status[ypos] == 0) {
	  qdif = zq.top() - pool[ypos].first;
	  break;
	}
	ypos -= 1;
      }
    }
    int sidx = -1;
    ll sdif = inf;
    if (pos < (int) ops.size()) {
      sidx = ops[pos].second;
      sdif = ops[pos].first;
      if (status[sidx] != 0) {
	sdif = inf;
      }
    }
    rem--;
    if (0) {
      if (qdif >= 0 && sdif >= 0) {
	if (pos >= (int) ops.size()) {
	  break;
	}
	pos++;
	continue;
      }
    }
    assert (min(qdif, sdif) != inf);
    if (qdif > sdif) {
      cerr << "sdif " << sidx << "=> " << sdif << " pos = " << pos << "\n";
      status[sidx] = 1;
      tot -= sdif;
      pos++;
      continue;
    }
    cerr << "qdif " << ypos << " => " << qdif << " zq.top = " << zq.top() << "\n";
    zq.pop();
    status[ypos] = 2;
    ypos -= 1;
    tot -= qdif;
  }
  assert (rem == 0);
  cout << tot << "\n";
  return 0;
}

int main_tle(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int x, y, z;
  cin >> x >> y >> z;
  vector<VL> a(x + y + z, VL(3));
  ll tot = 0;
  REP(i, 0, x + y + z) {
    REP(j, 0, 3) {
      cin >> a[i][j];
    }
    tot += max(a[i][0], max(a[i][1], a[i][2]));
  }
  int tolCount = 0;
  VI ten(3);
  ten[0] = x;
  ten[1] = y;
  ten[2] = z;
  while (true) {
    vector<VL> bound(3, VL());
    VI cnt(3, 0);
    REP(i, 0, x + y + z) {
      vector<PLI> tt;
      REP(j, 0, 3) {
	tt.push_back(PLI(a[i][j], j));
      }
      sort(tt.rbegin(), tt.rend());
      if (tt[0].first > tt[1].first) {
	int idx = tt[0].second;
	cnt[idx] += 1;
	bound[idx].push_back(tt[0].first - tt[1].first);
      }
    }
    bool found = false;
    REP(k, 0, 3) {
      if (ten[k] < cnt[k] || (tolCount > 0 && ten[k] == cnt[k])) {
	sort(bound[k].rbegin(), bound[k].rend());
	REP(i, 0, x + y + z) {
	  a[i][k] -= bound[k][ten[k] - 1];
	}
	REP(i, ten[k], cnt[k]) {
	  tot -= bound[k][i];
	}
	found = true;
	if (ten[k] == cnt[k]) tolCount--;
      }
      if (found) break;
    }
    if (not found) break;
  }
  cout << tot << "\n";
  return 0;
}
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int x, y, z;
  cin >> x >> y >> z;
  vector<VL> a(x + y + z, VL(3));
  vector<PLI> xz;
  REP(i, 0, x + y + z) {
    REP(j, 0, 3) {
      cin >> a[i][j];
    }
    xz.push_back(PLI(a[i][1] - a[i][2], i));
  }
  sort(xz.rbegin(), xz.rend());
  VL front(x + y + z);
  {
    ll tot = 0;
    priority_queue<ll, vector<ll>, greater<ll> > que;
    REP(i, 0, x + y + z) {
      int idx = xz[i].second;
      ll diff = a[idx][1] - a[idx][0];
      tot += diff;
      que.push(diff);
      if ((int) que.size() > y) {
	tot -= que.top();
	que.pop();
      }
      front[i] = tot;
    }
  }
  VL back(x + y + z);
  {
    ll tot = 0;
    priority_queue<ll, vector<ll>, greater<ll> > que;
    for (int i = x + y + z - 1; i >= 0; --i) {
      int idx = xz[i].second;
      ll diff = a[idx][2] - a[idx][0];
      tot += diff;
      que.push(diff);
      if ((int) que.size() > z) {
	tot -= que.top();
	que.pop();
      }
      back[i] = tot;
    }
  }
  ll ma = -inf;
  REP(i, y - 1, x + y) {
    ma = max(ma, front[i] + back[i + 1]);
  }
  REP(i, 0, x + y + z) {
    ma += a[i][0];
  }
  cout << ma << "\n";
}
