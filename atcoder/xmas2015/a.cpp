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


ll x, t, a, b, c;

typedef pair<ll, ll> pll;
void mul(pair<ll, ll> &a, const pair<ll, ll> &b) {
  ll u = a.first * b.first % c;
  ll v = (a.second * b.first + b.second) % c;
  a.first = u;
  a.second = v;
}

pll calc(ll idx) {
  pll sum = pll(1, 0);
  pll cur = pll(a, b);
  while (idx > 0) {
    if (idx % 2) {
      mul(sum, cur);
    }
    mul(cur, pll(cur));
    idx /= 2;
  }
  return sum;
}

int main(void){
  int n;
  cin >> n >> x >> t >> a >> b >> c;
  ll sum = 0;
  pll acc = pll(1, 0);
  pll by = calc(t);
  REP(i, 0, n) {
    ll res = acc.first * x % c;
    res += acc.second;
    res %= c;
    sum += res;
    mul(acc, by);
  }
  cout << sum << endl;
}
