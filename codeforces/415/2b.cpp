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
  int n, f;
  cin >> n >> f;
  VL pool(n);
  ll tot = 0;
  REP(i, 0, n) {
    ll k, l;
    cin >> k >> l;
    ll tmp1 = min(k, l);
    ll tmp2 = min(2 * k, l);
    tot += tmp1;
    pool[i] = tmp2 - tmp1;
  }
  sort(pool.rbegin(), pool.rend());
  REP(i, 0, f) {
    tot += pool[i];
  }
  cout << tot << endl;
}
