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

vector<double> row_table(int n, int r) {
  assert (n >= r);
  vector<double> ret(n);
  REP(i, 0, n - r + 1) {
    ret[i] += 1.0;
    if (i + r < n) {
      ret[i + r] -= 1;
    }
  }
  REP(i, 1, n) {
    ret[i] += ret[i - 1];
  }
  REP(i, 0, n) {
    ret[i] /= n - r + 1;
  }
  sort(ret.rbegin(), ret.rend());
  return ret;
}

int main(void) {
  int n, m, k, r;
  scanf("%d%d%d%d",&n,&m,&r,&k);
  vector<double> row = row_table(n, r);
  vector<double> col = row_table(m, r);
  double tot = 0.0;
  priority_queue<pair<double, PI>,
		 vector<pair<double, PI> >,
		 less<pair<double, PI> > > que;
  que.push(make_pair(row[0] * col[0], PI(0, 0)));
  REP(i, 0, k) {
    // find the i-th largest one
    assert (not que.empty());
    pair<double, PI> t = que.top(); que.pop();
    tot += t.first;
    int x = t.second.first;
    int y = t.second.second;
    if (y == 0 && x < n - 1) {
      que.push(make_pair(row[x + 1] * col[0], PI(x + 1, 0)));
    }
    if (y < m - 1) {
      que.push(make_pair(row[x] * col[y + 1], PI(x, y + 1)));
    }
  }
  printf("%.15f\n", tot);
}
