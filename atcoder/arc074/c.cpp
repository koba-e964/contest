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

ll calc(ll h, ll w) {
  ll mi = 1e18;
  REP(i, 1, h) {
    // try division
    vector<ll> tmp;
    ll t = (ll) i * w;
    tmp.push_back(t);
    ll qu = h - i;
    ll ww = w;
    if (ww < qu) {
      swap(qu, ww);
    }
    if (qu % 2 == 0) {
      swap(ww, qu);
    }
    tmp.push_back(qu * (ww / 2));
    tmp.push_back(qu * (ww - ww / 2));
    sort(tmp.begin(), tmp.end());
    mi = min(mi, tmp[2] - tmp[0]);
  }
  return mi;
}

int main(void){
  ll h, w;
  cin >> h >> w;
  ll mi = 1e18;
  mi = min(calc(h, w), mi);
  mi = min(calc(w, h), mi);
  cout << mi << endl;
}
