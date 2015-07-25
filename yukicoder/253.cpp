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

ll query(ll lo, ll hi, int &nq) {
  while (hi - lo >= 2) {
    ll mid = (hi + lo) / 2;
    cout << "? " << mid - nq << endl;
    int res;
    cin >> res;
    nq++;
    if (res == 0) {
      return mid;
    }
    if (res > 0) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  return lo;
}

int main(void){
  cin.sync_with_stdio(false);
  int nq = 0;
  ll res = query(10, 1024, nq);
  if (res <= 1000) {
    cout << "! " << res << endl;
    return 0;
  }
  cout << "! " << query(100, 1e9 + 1, nq) << endl;
}
