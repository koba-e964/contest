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

VL nico;

const int N = 20000000;
ll memo[N];

ll f(ll n) {
  if (n < N && memo[n] >= 0) {
    return memo[n];
  }
  ll mi = n;
  vector<pair<ll, ll> > cand;
  REP(i, 0, nico.size()) {
    ll q = nico[i];
    if (n >= q) {
      cand.push_back(make_pair(n % q, n / q));
    }
  }
  sort(cand.begin(), cand.end());
  REP(i, 0, cand.size()) {
    pair<ll, ll> c = cand[i];
    ll q = c.second;
    if (mi > c.first) {
      mi = min(mi, f(q) + c.first);
    }
  }
  if (n < N) {
    memo[n] = mi;
  }
  return mi;
}

int main(void){
  ll n;
  cin >> n;
  REP(i, 0, N) {
    memo[i] = -1;
  }
  ll c = 0;
  REP(i, 0, 9) {
    c = 10 * c + 2;
    nico.push_back(c);
    c = 10 * c + 5;
    nico.push_back(c);
  }
  c = 0;
  REP(i, 0, 9) {
    c = 10 * c + 5;
    nico.push_back(c);
    c = 10 * c + 2;
    nico.push_back(c);
  }
  sort(nico.begin(), nico.end());
  cout << f(n) << endl;
}
