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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 50;
int score(vector<VI> a) {
  // Perform a free fall, then calculate the cost
  REP(i, 0, N) {
    VI que;
    for (int j = N - 1; j >= 0; --j) {
      if (a[j][i] == 0) {
	continue;
      }
      if (a[j][i] == 4) {
	while (que.size() < N - j - 1) {
	  que.push_back(0);
	}
	que.push_back(4);
	continue;
      }
      que.push_back(a[j][i]);
    }
    assert (que.size() <= N);
    REP(j, 0, que.size()) {
      a[N - j - 1][i] = que[j];
    }
    REP(j, que.size(), N) {
      a[N - j - 1][i] = 0;
    }
  }
  // Calculates the score of a
  int o_conn = 0;
  int x_conn = 0;
  UnionFind uf(N * N);
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (1 <= a[i][j] && a[i][j] <= 2) {
	int dx[4] = {1, 0, -1, 0};
	int dy[4] = {0, 1, 0, -1};
	REP(d, 0, 4) {
	  int nx = i + dx[d];
	  int ny = j + dy[d];
	  if (nx < 0 || nx >= N || ny < 0 || ny >= N) {
	    continue;
	  }
	  if (a[i][j] == a[nx][ny]) {
	    uf.unite(i * N + j, nx * N + ny);
	  }
	}
      }
    }
  }
  VI acc(N * N);
  REP(i, 0, N * N) {
    acc[uf.root(i)] += 1;
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (a[i][j] == 1) {
	o_conn = max(o_conn, acc[i * N + j]);
      }
      if (a[i][j] == 2) {
	x_conn = max(x_conn, acc[i * N + j]);
      }
    }
  }
  return o_conn + x_conn;
  
}
/*
 * 0: .
 * 1: o
 * 2: x
 * 3: +
 * 4: -
 */
vector<VI> solve(vector<VI>);


void checker(void) {
  srand(0x12312);
  double sum = 0;
  double sq_sum = 0;
  REP(trial, 0, 2000) {
    vector<VI> a(N, VI(N));
    REP(i, 0, N) {
      REP(j, 0, N) {
	int v = rand() % 4;
	a[i][j] = max(v - 1, 0);
      }
    }
    vector<VI> sol = solve(a);
    int sc = score(sol);
    sum += sc;
    sq_sum += (double)sc * (double)sc;
    double avrg = sum / (trial + 1);
    double variance = sq_sum / (trial + 1) - avrg * avrg;
    if (trial >= 1) {
      double avrg_uncertainty = sqrt(variance / trial);
      if (trial % 1 == 0 || avrg_uncertainty <= 20) {
	cout << " [" << trial + 1 << "] estimated avrg = " << avrg << " \\pm "
	   << avrg_uncertainty << endl;
	if (trial >= 5 && avrg_uncertainty <= 20) {
	  cout << " estimated stdev = "
	       << sqrt(variance * (trial + 1) / trial) << endl;
	  break;
	}
      }
    }
  }
}

vector<VI> solve(vector<VI> a) {
  REP(loop_cnt, 0, 15) {
    REP(j, 0, N) {
      REP(i, 0, N) {
	if (a[i][j] != 0) {
	  continue;
	}
	int sc0 = score(a);
	a[i][j] = 3;
	int sc3 = score(a);
	int sc4 = 0;
	if (i >= 1 && a[i - 1][j] == 4) {}
	else {
	  a[i][j] = 4;
	  sc4 = score(a);
	}
	int ma = 0;
	int s = sc0;
	if (s < sc3) {
	  ma = 3;
	  s = sc3;
	}
	if (s < sc4) {
	  ma = 4;
	  s = sc4;
	}
	a[i][j] = ma;
      }
    }
  }
  return a;
}


int main(void){
#ifndef DEBUG
  vector<VI> a(N, VI(N));
  REP(i, 0, N) {
    string s;
    cin >> s;
    REP(j, 0, N) {
      a[i][j] = s[j] == '.' ? 0 :
	s[j] == 'o' ? 1 : 2;
    }
  }
  vector<VI> sol = solve(a);
  REP(i, 0, N) {
    REP(j, 0, N) {
      int v = sol[i][j];
      if (v == 0) {
	cout << ".";
      }
      if (v == 1) {
	cout << "o";
      }
      if (v == 2) {
	cout << "x";
      }
      if (v == 3) {
	cout << "+";
      }
      if (v == 4) {
	cout << "-";
      }
    }
    cout << endl;
  }
#else
  checker();
#endif
}
