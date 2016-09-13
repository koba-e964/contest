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


ll check(ll n, ll b) {
  ll s = 0;
  if (b <= 1) {
    return -1;
  }
  while (n > 0) {
    s += n % b;
    n /= b;
  }
  return s;
}

int main(void){
  ll n, s;
  cin >> n >> s;
  if (n < s) {
    cout << -1 << endl;
    return 0;
  }
  if (n == s) {
    cout << n + 1 << endl;
    return 0;
  }
  VL div;
  REP(b, 1, 400000) {
    if ((n - s) % b == 0) {
      div.push_back(b);
      div.push_back((n - s) / b);
    }
  }
  sort(div.begin(), div.end());
  REP(i, 0, div.size()) {
    if (s == check(n, div[i] + 1)) {
      cout << div[i] + 1 << endl;
      return 0;
    }
  }
  cout << -1 << endl;
}
