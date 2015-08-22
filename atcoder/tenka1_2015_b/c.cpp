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


const ll mod = 1e9 + 7;

int main(void){
  ll l;
  cin >> l;
  ll k = l / 4;
  ll r = l % 4;
  ll cnt = 2 * k - 3;
  if (l <= 8) {
    cout << 0 << endl;
    return 0;
  }
  cnt %= mod;
  cnt *= k % mod;
  cnt %= mod;
  if (r >= 2) {
    cnt += mod - 1;
  }
  cnt += r * k;
  cnt %= mod;
  REP(c, -1, 2) {
    ll rem = (l - 1 - c) / 3 - 1;
    cnt -= rem % mod;
    cnt += mod;
    cnt %= mod;
  }
  cnt++; // y = x-1
  cnt %= mod;
  cout << cnt << endl;
  return 0;
}
