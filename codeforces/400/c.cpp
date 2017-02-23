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
#include <unordered_map>
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
  int n;
  ll k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL pows;
  if (k * k == 1) {
    pows.push_back(1);
    if (k == -1) {
      pows.push_back(-1);
    }
  } else {
    ll cur = 1;
    while (abs(cur) <= 1e15) {
      pows.push_back(cur);
      cur *= k;
    }
  }

  VL s(n + 1, 0);
  REP(i, 0, n) {
    s[i + 1] = s[i] + a[i];
  }
  map<ll, int> val;
  ll cnt = 0;
  for (int i = n - 1; i >= 0; --i) {
    if (val.count(s[i + 1]) == 0) {
	val[s[i + 1]] = 0;
    }
    val[s[i + 1]] += 1;
    for (auto v: pows) {
      ll t = s[i] + v;
      cnt += val.count(t) ? val[t] : 0;
    }
  }
  cout << cnt << endl;
}
