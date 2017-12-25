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

vector<VI> transp(const vector<VI> &a) {
  int n = a.size();
  int m = a[0].size();
  vector<VI> ret(m, VI(n, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      int v = a[i][j] - 1;
      int vi = v / m;
      int vj = v % m;
      ret[j][i] = vj * n + vi + 1;
    }
  }
  return ret;
}

void output(const vector<VI> &a) {
  cout << "YES\n";
  int n = a.size();
  int m = a[0].size();
  REP(i, 0, n) {
    REP(j, 0, m) {
      cout << a[i][j] << (j == m - 1 ? "\n" : " ");
    }
  }
  exit(0);
}

void fail(void) {
  cout << "NO\n";
  exit(0);
}


// m >= 4
vector<VI> cons(int n, int m) {
  assert (m >= 4);
  vector<VI> ret(n, VI(m, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      ret[(i + (j % 2 == 0 ? 1 : 0)) % n][(j / 2) + (j % 2 == 0 ? m / 2 : 0)] = i * m + j + 1;
    }
  }
  REP(i, 0, n) {
    if (i % 2 == 1) {
      rotate(ret[i].begin(), ret[i].begin() + 1, ret[i].begin() + m / 2);
      rotate(ret[i].begin() + m / 2, ret[i].end() - 1, ret[i].end());
    }
  }
  return ret;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  if (m >= 4) {
    output(cons(n, m));
  }
  if (n >= 4) {
    output(transp(cons(m, n)));
  }
  VI a(n * m);
  REP(i, 0, n * m) {
    a[i] = i + 1;
  }
  bool found = false;
  set<PI> edges;
  REP(i, 0, n - 1) {
    REP(j, 0, m) {
      edges.insert(PI(i * m + j + 1, (i + 1) * m + j + 1));
    }
  }
  REP(i, 0, n) {
    REP(j, 0, m - 1) {
      edges.insert(PI(i * m + j + 1, i * m + (j + 1) + 1));
    }
  }
  do {
    bool ok = true;
    REP(i, 0, n - 1) {
      REP(j, 0, m) {
	int u = a[i * m + j];
	int v = a[(i + 1) * m + j];
	if (u > v) swap(u, v);
	if (edges.count(PI(u, v))) {
	  ok = false;
	  break;
	}
      }
    }
    REP(i, 0, n) {
      REP(j, 0, m - 1) {
	int u = a[i * m + j];
	int v = a[i * m + (j + 1)];
	if (u > v) swap(u, v);
	if (edges.count(PI(u, v))) {
	  ok = false;
	  break;
	}
      }
    }
    if (ok) {
      found = true;
      break;
    }
  } while (next_permutation(a.begin(), a.end()));
  if (not found) {
    fail();
  }
  vector<VI> ret(n, VI(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      ret[i][j] = a[i * m + j];
    }
  }
  output(ret);
}
