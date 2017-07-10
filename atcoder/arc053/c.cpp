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
typedef pair<ll, ll> PI;


ll sign(ll x) {
  return x < 0 ? -1 : x == 0 ? 0 : 1;
}

struct cmp {
  bool operator()(PI a, PI b) const {
    ll adiff = a.first - a.second;
    ll bdiff = b.first - b.second;
    if (sign(adiff) != sign(bdiff) || sign(adiff) * sign(bdiff) == 0) {
      return adiff < bdiff;
    }
    ll former = max(a.first, adiff + b.first);
    ll latter = max(b.first, bdiff + a.first);
    return former < latter;
  }
};


int main(void){
  int n;
  cin >> n;
  vector<PI> ab;
  REP(i, 0, n) {
    ll a, b;
    cin >> a >> b;
    ab.push_back(PI(a, b));
  }
  sort(ab.begin(), ab.end(), cmp());
  ll ma = 0;
  ll cur = 0;
  REP(i, 0, n) {
    cur += ab[i].first;
    ma = max(ma, cur);
    cur -= ab[i].second;
  }
  cout << ma << endl;
}
