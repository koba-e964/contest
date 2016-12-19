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
  int n, q;
  cin >> n >> q;
  VI last(n, 0);
  REP(loop_cnt, 0, q) {
    int t, k, d;
    cin >> t >> k >> d;
    VI res;
    REP(i, 0, n) {
      if (last[i] <= t) {
	res.push_back(i);
      }
    }
    if (res.size() < k) {
      cout << -1 << endl;
    } else {
      int tot = 0;
      REP(i, 0, k) {
	tot += res[i] + 1;
	last[res[i]] = t + d;
      }
      cout << tot << endl;
    }
  }
}
