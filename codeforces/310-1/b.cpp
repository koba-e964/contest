#include <algorithm>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N = 2e5 + 10;
ll l[N], r[N];

typedef pair<ll, ll> PL;

/*
 * interval : vector of pair<ll, ll>. pair(a, b) represents interval [a, b].
 * pts: points
 * returns allocation if possible, and empty vector if not possible.
 * allocation: interval[i] contains pts[ret[i]]
 */
vector<int> interval_matching(const vector<pair<ll, ll> > &interval,
			      const vector<ll> &pts) {
  typedef pair<ll, ll> PL;
  const int n = interval.size();
  const int m = pts.size();
  int pos = 0;
  vector<pair<PL,int> > dist(n);
  vector<pair<ll, int> > a(m);
  vector<int> sol(n);
  for (int i = 0; i < n; i++) {
    dist[i] = pair<PL, int>(interval[i], i);
  }
  for (int i = 0; i < m; ++i) {
    a[i] = pair<ll, int>(pts[i], i);
  }
  sort(dist.begin(), dist.end());
  sort(a.begin(), a.end());
  priority_queue<PL, vector<PL>, greater<PL> > que;
  for (int ai = 0; ai < m; ++ai) {
    PL c = a[ai];
    while (pos < n && dist[pos].first.first <= c.first) {
      que.push(PL(dist[pos].first.second, dist[pos].second));
      pos++;
    }
    if (que.empty()) {
      continue;
    }
    PL top = que.top();
    if (c.first > top.first) {
      break;
    }
    sol[top.second] = c.second;
    que.pop();
  }
  if (pos == n && que.empty()) {
    return sol;
  }
  return vector<int>();
}


int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> l[i] >> r[i];
  }
  vector<ll> pts(m);
  REP(i, 0, m) {
    ll q;
    cin >> q;
    pts[i] = q;
  }
  vector<PL> intv(n - 1);
  REP(i, 0, n - 1) {
    intv[i] = PL(l[i + 1] - r[i], r[i + 1] - l[i]);
  }
  vector<int> sol = interval_matching(intv, pts);
  if (sol.size()) {
    cout << "Yes" << endl;
    REP(i, 0, n - 1) {
      cout << sol[i] + 1;
      if (i < n - 1) cout << " ";
    }
    cout << endl;
  } else {
    cout << "No" << endl;
  }
}
