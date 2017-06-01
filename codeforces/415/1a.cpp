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
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL x(n);
  REP(i, 0, n) {
    cin >> x[i];
  }
  sort(x.begin(), x.end());
  ll tot = 0;
  ll cur = 1;
  REP(i, 0, n) {
    ll tmp = (cur + mod - 1) % mod;
    tot = (tot + x[i] * tmp) % mod;
    tot = (tot + x[n - i - 1] * (mod - tmp)) % mod;
    cur *= 2;
    cur %= mod;
  }
  cout << tot << endl;
}
