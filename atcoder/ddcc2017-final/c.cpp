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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;

const int N = 610;
const int M = 90000;
vector<PI> edges[N];

int a[M], b[M];
ll c[M];

bool vis[N];
ll pot[N];
int pkind[N];

vector<PIL> pool[N];
int maxp = 0;


const int DEBUG = 0;


bool dfs(int v, int ex, ll p, int kind, int preve) {
  if (DEBUG) {
    cerr << "dfs " << v << " " << ex << " " << p << endl;
  }
  if (vis[v]) {
    ll c = p - pot[v];
    if (DEBUG) {
      cerr << "branch: " << p << " " << kind << endl;
      cerr << "old: " << pot[v] << " " << pkind[v] << endl;
      cerr << "diff = " << c << endl;
    }
    if (pkind[v] == kind) {
      return c == 0;
    }
    maxp = max(maxp, max(kind, pkind[v]) + 2);
    pool[kind + 1].push_back(PIL(pkind[v] + 1, c));
    pool[pkind[v] + 1].push_back(PIL(kind + 1, -c));
    return true;
  }
  vis[v] = true;
  pot[v] = p;
  pkind[v] = kind;
  REP(i, 0, edges[v].size()) {
    PI dat = edges[v][i];
    int w = dat.first;
    int e = dat.second;
    bool neg = e < 0;
    if (e < 0) {
      e = -e - 1;
    }
    if (e == preve) {
      continue;
    }
    ll weight = neg ? -c[e] : c[e];
    bool res = dfs(w, ex, p + weight, v == ex ? i : kind, e);
    if (!res) {
      return false;
    }
  }
  return true;
}


ll tp[N];
int tvis[N];

VL tmp;
bool dfs2(int v, ll p) {
  if (DEBUG)
    cerr << "dfs2 " << v << " " << p << endl;
  if (tvis[v]) {
    if (tp[v] != p && DEBUG) {
      cerr << "contradict " << v << " " << tp[v] << " " << p << endl;
    }
    return tp[v] == p;
  }
  tvis[v] = true;
  tp[v] = p;
  tmp.push_back(p);
  REP(i, 0, pool[v].size()) {
    int w = pool[v][i].first;
    ll dist = pool[v][i].second;
    bool res = dfs2(w, p + dist);
    if (!res) {
      return false;
    }
  }
  return true;
}

int minority(VL v) {
  if (DEBUG) {
    cerr << "minor";
    for (auto i: v) cerr << " " << i;
    cerr << endl;
  }
  int n = v.size();
  if (n <= 1) {
    return 0;
  }
  if (n == 2) {
    return v[0] != v[1];
  }
  sort(v.begin(), v.end());
  if (v[0] == v[n - 2]) {
    return v[0] != v[n - 1];
  }
  if (v[1] == v[n - 1]) {
    return v[0] != v[n - 1];
  }
  return 2;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i];
    a[i]--, b[i]--;
    edges[a[i]].push_back(PI(b[i], i));
    edges[b[i]].push_back(PI(a[i], -i - 1));
  }
  REP(i, 0, n) {
    // Exclude the edge i, dfs
    REP(j, 0, n) {
      vis[j] = false;
      pot[j] = 0;
      maxp = 0;
    }
    REP(i, 0, N) {
      tvis[i] = false;
      tp[i] = 0;
      pool[i].clear();
    }
    bool result = dfs(i, i, 0, -1, -1);
    assert (maxp < N);
    int difc = 0;
    REP(j, 0, maxp) {
      if (tvis[j]) {continue;}
      tmp.clear();
      result &= dfs2(j, 0);
      difc += minority(tmp);
      if (DEBUG) {
	cerr << "difc = " << difc << endl;
      }
    }
    if (difc >= 2) {
      if (DEBUG) {
	cerr << result << " " << "hit" << endl;
      }
      result = false;
    }
    if (DEBUG) {
      cerr << "ex = " << i << endl;
      REP(i, 0, maxp) {
	REP(j, 0, pool[i].size()) {
	  PIL d = pool[i][j];
	  cerr << i << "->" << d.first << " " << d.second << endl;
	}
      }
      cerr << endl;
      REP(i, 0, n) {
	cerr << "pot[" << i << "]=" << pot[i] << "\n";
	cerr << "pkind[" << i << "]=" << pkind[i] << "\n";
	cerr << "tp[" << i << "]=" << tp[i] << endl;
      }
    }
    if (result) {
      cout << "Yes\n";
      return 0;
    }
  }
  cout << "No\n";
}
