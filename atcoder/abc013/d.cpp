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
typedef pair<int, int> PI;
const double EPS=1e-9;


VI compose(const VI &p1, const VI &p2) {
  int n = p1.size();
  VI res(n);
  REP(i, 0, n) {
    res[i] =  p1[p2[i]];
  }
  return res;
}

int main(void){
  int n, m, d;
  cin >> n >> m >> d;
  VI perm(n);
  REP(i, 0, n) {
    perm[i] = i;
  }
  VI a(m);
  REP(i, 0, m) {
    cin >> a[i];
  }

  for(int i = m - 1; i >= 0; --i) {
    swap(perm[a[i] - 1], perm[a[i]]);
  }

  VI sum(n);
  REP(i, 0, n) sum[i] = i;

  VI cur(perm.begin(), perm.end());

  while (d > 0) {
    if (d % 2 == 1) {
      sum = compose(sum, cur);
    }
    cur = compose(cur, cur);
    d /= 2;
  }

  REP(i, 0, n) {
    cout << sum[i] + 1 << endl;
  }
}
