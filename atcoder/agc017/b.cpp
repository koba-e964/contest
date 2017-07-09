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

bool calc(ll n, ll b, ll c, ll d) {
  REP(i, 0, n + 1) {
    // i positive, (n - i) negative
    ll hi = i * d - (n - i) * c;
    ll lo = i * c - (n - i) * d;
    if (lo <= b && b <= hi) {
      return true;
    }
  }
  return false;
}

int main(void){
  int n;
  ll a, b, c, d;
  cin >> n >> a >> b >> c >> d;
  cout << (calc(n - 1, b - a, c, d) ? "YES" : "NO") << endl;
}
