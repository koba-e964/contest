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

const int DEBUG = 0;

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};


pair<bool, char> is_same_color(const vector<string> &a, int top, int bottom, int l, int r, int bits) {
  int c = a[0].size();
  set<char> col;
  REP(i, top, bottom) {
    REP(j, l, r) {
      int x = i * c + j;
      if (bits & 1 << x) {
	col.insert(a[i][j]);
      }
    }
  }
  char ch = '\0';
  if (col.size() == 1) ch = *col.begin();
  return make_pair(col.size() <= 1, ch);
}


int solve_naive(const vector<string> &a) {
  int r = a.size();
  int c = a[0].size();
  assert (r * c <= 12);
  int ma = 0;
  REP(bits, 0, 1 << (r * c)) {
    // connected?
    set<int> pop;
    UnionFind uf(r * c);
    REP(i, 0, r) {
      REP(j, 0, c - 1) {
	int x = i * c + j;
	int y = i * c + j + 1;
	if ((bits & 1 << x) && (bits & 1 << y)) {
	  uf.unite(x, y);
	}
      }
    }
    REP(i, 0, r - 1) {
      REP(j, 0, c) {
	int x = i * c + j;
	int y = (i + 1) * c + j;
	if ((bits & 1 << x) && (bits & 1 << y)) {
	  uf.unite(x, y);
	}
      }
    }
    REP(i, 0, r * c) {
      if (bits & 1 << i) {
	pop.insert(uf.root(i));
      }
    }
    if (pop.size() != 1) continue;
    // Is the pattern valid?
    bool valid = false;
    REP(x, 0, r + 1) {
      if (valid) break;
      REP(y, 0, c + 1) {
	pair<bool, char> t1 = is_same_color(a, 0, x, 0, y, bits);
        pair<bool, char> t2 = is_same_color(a, 0, x, y, c, bits);
	pair<bool, char> t3 = is_same_color(a, x, r, 0, y, bits);
	pair<bool, char> t4 = is_same_color(a, x, r, y, c, bits);
	if (t1.first && t2.first && t3.first && t4.first) {
	  // check
	  vector<string> tt(2, string(2, '+'));
	  tt[0][0] = t1.second;
	  tt[0][1] = t2.second;
	  tt[1][0] = t3.second;
	  tt[1][1] = t4.second;
	  bool found = false;
	  REP(i, 0, r + 1) {
	    REP(j, 0, c + 1) {
	      bool match = true;
	      REP(k, 0, 2) {
		REP(l, 0, 2) {
		  if (tt[k][l] == '\0') continue;
		  int nx = i + k - 1;
		  int ny = j + l - 1;
		  if (nx < 0 || nx >= r || ny < 0 || ny >= c) {
		    match = false;
		    break;
		  }
		  if (tt[k][l] != a[nx][ny]) {
		    match = false;
		    break;
		  }
		}
	      }
	      if (match) {
		found = true;
	      }
	    }
	  }
	  if (found) {
	    valid = true;
	    break;
	  }
	}
      }
    }
    if (DEBUG && valid) {
      cerr << "pat:" << endl;
      REP(i, 0, r) {
	REP(j, 0, c) {
	  if (bits & 1 << (i * c + j)) {
	    cerr << a[i][j];
	  } else {
	    cerr << " ";
	  }
	}
	cerr << endl;
      }
    }
    if (valid) {
      ma = max(ma, __builtin_popcount(bits));
    }
  }
  return ma;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int r, c;
    cin >> r >> c;
    vector<string> a(r);
    REP(i, 0, r) cin >> a[i];
    int ans = solve_naive(a);
    cout << "Case #" << case_nr << ": " << ans << endl;
  }
}
