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
const ll mod = 1e9 + 7;

// Rearrange elements at (dat[i])_i and minimize the total distance
ll calc(const VI &dat) {
  int m = dat.size();
  VI b(m);
  REP(i, 0, m) {
    b[dat[i]]++;
  }
  ll sum = 0;
  int cur = 0;
  REP(i, 0, m) {
    cur += b[i] - 1;
    sum += abs(cur);
  }
  assert (cur == 0);
  return sum;
}

typedef pair<int, PI> PIPI;
int main(void){
  int n, m, x0, a, p;
  cin >> n >> m >> x0 >> a >> p;
  vector<PIPI> x(n * m);
  REP(i, 0, n * m) {
    x[i] = PIPI(x0, PI(i / m, i % m));
    x0 = (x0 + a) % p;
  }
  sort(x.begin(), x.end());
  ll sum = 0;
  REP(i, 0, n) {
    VI dat(m);
    REP(j, 0, m) {
      PIPI cur = x[i * m + j];
      sum += abs(i - cur.second.first);
      dat[j] = cur.second.second;
    }
    sum += calc(dat);
  }
  cout << sum << endl;
}
