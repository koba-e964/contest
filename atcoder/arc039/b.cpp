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

const ll mod = 1e9+7;

ll invmod(ll x) {
  ll n = mod - 2;
  ll sum = 1, cur = x;
  while (n > 0) {
    if (n % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    n /= 2;
  }
  return sum;
}


ll comb(int x, int y) {
  ll sum = 1;
  REP(i, 0, y) {
    sum *= x - i;
    sum %= mod;
    sum = sum * invmod(i + 1) % mod;
  }
  return sum;
}

int main(void){
  int n,k;
  cin >> n >> k;
  if (n <= k) {
    cout << comb(n, k%n) << endl;
  } else {
    cout << comb(k + n - 1, k) << endl;
  }
}
