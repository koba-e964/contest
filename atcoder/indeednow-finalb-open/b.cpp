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



int main(void){
  int n;
  cin >> n;
  vector<PI> pool;
  REP(i, 0, n) {
    int s, t;
    cin >> s >> t;
    pool.push_back(PI(s, 2 * i));
    pool.push_back(PI(t, 2 * i + 1));
  }
  sort(pool.begin(), pool.end());
  VI res(n, -114514);
  int cnt = 0;
  REP(i, 0, pool.size()) {
    PI w = pool[i];
    int kind = w.second % 2;
    int v = w.second / 2;
    if (kind == 0) {
      res[v] = -cnt;
      cnt += 1;
    } else {
      res[v] += cnt - 1;
    }
  }
  REP(i, 0, n) {
    cout << res[i] << endl;
  }
}
