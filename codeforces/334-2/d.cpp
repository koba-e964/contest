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
ll pow(ll x, ll y) {
  ll sum = 1;
  for (ll i = 0; i < y; ++i) {
    sum *= x;
    sum %= mod;
  }
  return sum;
}

int main(void){
  ll k, p;
  cin >> p >> k;
  if (k == 0) { // any f s.t. f(0) = 0
    cout << pow(p, p - 1) << endl;
    return 0;
  }
  if (k == 1) {// any f
    cout << pow(p, p) << endl;
    return 0;
  }
  // determine ord(k)
  int o = 1;
  ll prod = k;
  while (prod != 1) {
    prod *= k;
    prod %= p;
    o++;
  }
  assert ((p - 1) % o == 0);
  cout << pow(p, (p - 1) / o) << endl;
}
